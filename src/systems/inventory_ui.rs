use bevy::prelude::*;
use crate::components::{
    components::{Player, Position},
    inventory::{Inventory, CarryCapacity},
    item::{GroundItems, Item},
};
use crate::resources::turn_state::TurnPhase;
use crate::systems::ui_kit::*;

/// Marker component for the inventory UI root
#[derive(Component)]
pub struct InventoryUiRoot;

/// Component tracking which item is selected (on the UI root)
#[derive(Component)]
pub struct InventorySelection {
    pub selected_index: usize,
}

/// Marker component for individual inventory item rows with their index
#[derive(Component)]
pub struct InventoryItemRow {
    pub index: usize,
}

/// Detects Tab key press and transitions to ViewingInventory phase
pub fn detect_inventory_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        next_phase.set(TurnPhase::ViewingInventory);
    }
}

fn inventory_row_text(index: usize, item: &Item) -> String {
    let value_str = match item.value {
        Some(v) => format!("Value: {}", v),
        None => "Tool".to_string(),
    };
    let metal_str = if item.is_metal { " [Metal]" } else { "" };
    format!(
        "{}. {} (Weight: {}, {}){}",
        index + 1,
        item.name,
        item.weight,
        value_str,
        metal_str
    )
}

/// Shared builder for the inventory modal (spawn and rebuild paths).
fn build_inventory_ui(
    commands: &mut Commands,
    inventory: &Inventory,
    selected: usize,
    current_weight: u32,
    max_capacity: u32,
    gravity_active: bool,
) -> Entity {
    let is_overweight = current_weight > max_capacity;

    spawn_modal(
        commands,
        (
            InventoryUiRoot,
            InventorySelection {
                selected_index: selected,
            },
        ),
        "Inventory",
        640.0,
        480.0,
        55.0,
        true,
        |content| {
            let weight_text = if is_overweight {
                format!("Weight: {}/{} (OVERWEIGHT!)", current_weight, max_capacity)
            } else {
                format!("Weight: {}/{}", current_weight, max_capacity)
            };
            modal_text(
                content,
                weight_text,
                FONT_SUB,
                if is_overweight { COL_FAIL } else { Color::srgb(0.7, 0.7, 0.7) },
            );
            if inventory.is_empty() {
                modal_text(content, "(Empty)", FONT_BODY, Color::srgb(0.5, 0.5, 0.5));
            }
            for (index, item) in inventory.items.iter().enumerate() {
                let bg_color = if index == selected {
                    Color::srgb(0.3, 0.5, 0.3) // Highlighted (green)
                } else {
                    Color::srgb(0.1, 0.1, 0.1) // Normal
                };
                content
                    .spawn((
                        Node {
                            padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(bg_color),
                        InventoryItemRow { index },
                    ))
                    .with_children(|row| {
                        modal_text(row, inventory_row_text(index, item), FONT_BODY, Color::srgb(0.9, 0.9, 0.9));
                    });
            }
            let _ = gravity_active; // reserved: capacity switch messaging
        },
        &[
            hint("W/S", "Select"),
            hint("D", "Drop"),
            hint("Esc", "Close"),
        ],
    )
}

/// Spawns the inventory UI when entering ViewingInventory phase
pub fn spawn_inventory_ui_system(
    mut commands: Commands,
    player_query: Query<(&Inventory, Option<&crate::components::components::GravitationalAnomalyTimer>), With<Player>>,
    capacity: Res<CarryCapacity>,
    existing_ui: Query<Entity, With<InventoryUiRoot>>,
) {
    // Don't spawn if UI already exists
    if existing_ui.iter().next().is_some() {
        return;
    }

    let Ok((inventory, gravity_timer)) = player_query.single() else {
        warn!("Failed to get player inventory!");
        return;
    };

    let max_capacity = if gravity_timer.is_some() {
        capacity.in_gravity
    } else {
        capacity.normal
    };

    build_inventory_ui(
        &mut commands,
        inventory,
        0,
        inventory.total_weight(),
        max_capacity,
        gravity_timer.is_some(),
    );
}

/// Despawns the inventory UI when exiting ViewingInventory phase
pub fn despawn_inventory_ui_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<InventoryUiRoot>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Handles ESC key to close inventory UI
/// Closing the inventory menu consumes 1 turn (transitions to WorldUpdate)
pub fn close_inventory_ui_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_phase.set(TurnPhase::WorldUpdate);
    }
}

/// Handles W/S and arrow navigation in inventory
pub fn inventory_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selection_query: Query<&mut InventorySelection>,
    player_query: Query<&Inventory, With<Player>>,
) {
    let Ok(mut selection) = selection_query.single_mut() else {
        return;
    };

    let Ok(inventory) = player_query.single() else {
        return;
    };

    if inventory.is_empty() {
        return;
    }

    let max_index = inventory.count() - 1;

    // S = down, W = up (consistent with movement); arrows work too.
    // Navigation wraps around at both ends.
    let down = keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown);
    let up = keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp);

    if down {
        selection.selected_index = if selection.selected_index >= max_index {
            0
        } else {
            selection.selected_index + 1
        };
    } else if up {
        selection.selected_index = if selection.selected_index == 0 {
            max_index
        } else {
            selection.selected_index - 1
        };
    }
}

/// Handles D key to drop selected item (always drops on player's current tile)
pub fn drop_item_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&mut Inventory, &Position), With<Player>>,
    selection_query: Query<&InventorySelection>,
    mut ground_items_query: Query<(Entity, &Position, &mut GroundItems), Without<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyD) {
        return;
    }

    let Ok((mut inventory, player_pos)) = player_query.single_mut() else {
        return;
    };

    let Ok(selection) = selection_query.single() else {
        return;
    };

    if inventory.is_empty() || selection.selected_index >= inventory.count() {
        return;
    }

    // Remove item from inventory
    let Some(dropped_item) = inventory.remove_item(selection.selected_index) else {
        return;
    };

    // Drop on player's current tile
    let drop_pos = *player_pos;

    // Find or create GroundItems entity at drop position
    let mut found = false;
    for (_, pos, mut ground_items) in ground_items_query.iter_mut() {
        if pos.x == drop_pos.x && pos.y == drop_pos.y {
            ground_items.add_item(dropped_item.clone());
            found = true;
            break;
        }
    }

    if !found {
        // Create new GroundItems entity
        let mut new_ground_items = GroundItems::new();
        new_ground_items.add_item(dropped_item.clone());
        commands.spawn((drop_pos, new_ground_items));
    }

    info!("Dropped {} at ({}, {})", dropped_item.name, drop_pos.x, drop_pos.y);
}

/// Updates UI highlighting based on selection
pub fn update_inventory_ui_selection_system(
    selection_query: Query<&InventorySelection>,
    mut item_rows_query: Query<(&InventoryItemRow, &mut BackgroundColor)>,
) {
    // Get current selection
    let Ok(selection) = selection_query.single() else {
        return;
    };

    // Update background color for all item rows every frame
    // (This is more reliable than Changed detection for UI updates)
    for (row, mut bg_color) in item_rows_query.iter_mut() {
        let new_color = if row.index == selection.selected_index {
            Color::srgb(0.3, 0.5, 0.3) // Highlighted (green)
        } else {
            Color::srgb(0.1, 0.1, 0.1) // Normal
        };

        // Only update if color actually changed to avoid unnecessary updates
        if bg_color.0 != new_color {
            *bg_color = BackgroundColor(new_color);
        }
    }
}

/// Scrolls the selected row into view after navigation or rebuild.
/// Uses measured layout (ComputedNode) instead of hardcoded row heights.
pub fn auto_scroll_inventory_system(
    selection_query: Query<&InventorySelection, Changed<InventorySelection>>,
    mut scroll_query: Query<(&ComputedNode, &mut ScrollPosition), With<ModalScrollArea>>,
    rows: Query<&InventoryItemRow>,
) {
    let Ok(selection) = selection_query.single() else {
        return;
    };
    let Ok((node, mut scroll)) = scroll_query.single_mut() else {
        return;
    };
    let count = rows.iter().count();
    if count == 0 {
        return;
    }
    let content_height = node.content_size().y * node.inverse_scale_factor;
    let view_height = node.size().y * node.inverse_scale_factor;
    let row_height = content_height / count as f32;
    scroll_selection_into_view(
        &mut *scroll,
        content_height,
        view_height,
        selection.selected_index,
        row_height,
    );
}

/// Rebuilds inventory UI when inventory changes (e.g., after dropping items)
pub fn rebuild_inventory_ui_system(
    mut commands: Commands,
    player_query: Query<(&Inventory, Option<&crate::components::components::GravitationalAnomalyTimer>), (With<Player>, Changed<Inventory>)>,
    ui_query: Query<Entity, With<InventoryUiRoot>>,
    selection_query: Query<&InventorySelection>,
    capacity: Res<CarryCapacity>,
) {
    // Only rebuild if inventory changed
    if player_query.is_empty() {
        return;
    }

    let Ok((inventory, gravity_timer)) = player_query.single() else {
        return;
    };

    // Save current selection index
    let selected_index = selection_query.single().map(|s| s.selected_index).unwrap_or(0);

    // Despawn old UI (if it still exists - it might have been despawned by state transition)
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }

    // Clamp selection to valid range
    let max_index = if inventory.is_empty() {
        0
    } else {
        inventory.count() - 1
    };
    let clamped_selection = selected_index.min(max_index);

    let max_capacity = if gravity_timer.is_some() {
        capacity.in_gravity
    } else {
        capacity.normal
    };

    // Rebuild UI with updated inventory
    build_inventory_ui(
        &mut commands,
        inventory,
        clamped_selection,
        inventory.total_weight(),
        max_capacity,
        gravity_timer.is_some(),
    );
}
