use bevy::prelude::*;
use crate::components::{
    components::Player,
    item::{GroundItems, Item},
    inventory::{Inventory, CarryCapacity},
};
use crate::resources::turn_state::TurnPhase;
use crate::resources::message_log::MessageLog;
use crate::systems::ground_items::GroundItemSprite;
use crate::systems::ui_kit::*;

/// Marker component for the inspect UI root
#[derive(Component)]
pub struct InspectUiRoot;

/// Marker component for individual item rows in inspect UI
#[derive(Component)]
pub struct InspectItemRow {
    pub index: usize,
}

/// Component tracking which item is selected for pickup (on the UI root)
#[derive(Component)]
pub struct InspectSelection {
    pub selected_index: usize,
}

/// Detects E key press and transitions to InspectingItems phase if player is on items tile
pub fn detect_inspect_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&crate::components::components::Position, With<Player>>,
    ground_items_query: Query<(&crate::components::components::Position, &GroundItems)>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        if let Ok(player_pos) = player_query.single() {
            // Check if there are items at player's position
            for (item_pos, ground_items) in ground_items_query.iter() {
                if item_pos.x == player_pos.x && item_pos.y == player_pos.y && !ground_items.is_empty() {
                    // Transition to InspectingItems phase
                    next_phase.set(TurnPhase::InspectingItems);
                    return;
                }
            }
        }
    }
}

fn inspect_row_text(index: usize, item: &Item) -> String {
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

/// Shared builder for the inspect modal (spawn and rebuild paths).
fn build_inspect_ui(
    commands: &mut Commands,
    items: &[Item],
    selected: usize,
    current_weight: u32,
    capacity_normal: u32,
) -> Entity {
    spawn_modal(
        commands,
        (
            InspectUiRoot,
            InspectSelection {
                selected_index: selected,
            },
        ),
        "Items on Ground",
        540.0,
        400.0,
        55.0,
        true,
        |content| {
            modal_text(
                content,
                format!("Current Weight: {}/{}", current_weight, capacity_normal),
                FONT_SUB,
                Color::srgb(0.7, 0.7, 0.7),
            );
            for (index, item) in items.iter().enumerate() {
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
                        InspectItemRow { index },
                    ))
                    .with_children(|row| {
                        modal_text(row, inspect_row_text(index, item), FONT_BODY, Color::srgb(0.9, 0.9, 0.9));
                    });
            }
        },
        &[
            hint("W/S", "Select"),
            hint("E", "Pick up"),
            hint("Esc", "Close"),
        ],
    )
}

/// Spawns the inspect UI when entering InspectingItems phase
pub fn spawn_inspect_ui_system(
    mut commands: Commands,
    player_query: Query<(&crate::components::components::Position, &Inventory), With<Player>>,
    ground_items_query: Query<(&crate::components::components::Position, &GroundItems)>,
    existing_ui: Query<Entity, With<InspectUiRoot>>,
    capacity: Res<CarryCapacity>,
) {
    // Don't spawn if UI already exists
    if existing_ui.iter().next().is_some() {
        return;
    }

    // Find items at player's position
    let Ok((player_pos, inventory)) = player_query.single() else {
        return;
    };

    let Some(items) = ground_items_query
        .iter()
        .find(|(pos, _)| pos.x == player_pos.x && pos.y == player_pos.y)
        .map(|(_, ground_items)| &ground_items.items)
    else {
        return;
    };

    build_inspect_ui(
        &mut commands,
        &items,
        0,
        inventory.total_weight(),
        capacity.normal,
    );
}

/// Despawns the inspect UI when exiting InspectingItems phase
pub fn despawn_inspect_ui_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<InspectUiRoot>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Handles ESC key to close inspect UI (when in InspectingItems phase)
/// Closing the inspect menu consumes 1 turn (transitions to WorldUpdate)
pub fn close_inspect_ui_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_phase.set(TurnPhase::WorldUpdate);
    }
}

/// Handles W/S and arrow navigation in inspect UI
pub fn inspect_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selection_query: Query<&mut InspectSelection>,
    player_query: Query<&crate::components::components::Position, With<Player>>,
    ground_items_query: Query<(&crate::components::components::Position, &GroundItems)>,
) {
    let Ok(mut selection) = selection_query.single_mut() else {
        return;
    };

    let Ok(player_pos) = player_query.single() else {
        return;
    };

    // Find items at player's position
    let Some((_, ground_items)) = ground_items_query
        .iter()
        .find(|(pos, _)| pos.x == player_pos.x && pos.y == player_pos.y)
    else {
        return;
    };

    if ground_items.is_empty() {
        return;
    }

    let max_index = ground_items.count() - 1;

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

/// Handles E key to pickup selected item
pub fn pickup_item_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&crate::components::components::Position, &mut Inventory), With<Player>>,
    mut ground_items_query: Query<(Entity, &crate::components::components::Position, &mut GroundItems)>,
    sprite_query: Query<(Entity, &GroundItemSprite)>,
    selection_query: Query<&InspectSelection>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
    mut message_log: ResMut<MessageLog>,
    mut commands: Commands,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok((player_pos, mut inventory)) = player_query.single_mut() else {
        return;
    };

    let Ok(selection) = selection_query.single() else {
        return;
    };

    // Find ground items at player position
    let mut ground_entity = None;
    let mut item_to_pickup = None;

    for (entity, pos, mut ground_items) in ground_items_query.iter_mut() {
        if pos.x == player_pos.x && pos.y == player_pos.y {
            if selection.selected_index < ground_items.count() {
                // Remove item from ground
                item_to_pickup = ground_items.remove_item(selection.selected_index);
                ground_entity = Some((entity, ground_items.is_empty()));
                break;
            }
        }
    }

    if let Some(item) = item_to_pickup {
        // Add to inventory (capacity is unlimited, but movement is blocked if over)
        message_log.add_message(format!("Picked up: {}", item.name));
        info!("Picked up: {} (weight: {})", item.name, item.weight);
        inventory.add_item(item);

        // If ground items are now empty, despawn the sprite and entity
        if let Some((entity, is_empty)) = ground_entity {
            if is_empty {
                // First despawn the sprite to prevent orphaning
                for (sprite_entity, sprite) in sprite_query.iter() {
                    if sprite.ground_items_entity == entity {
                        commands.entity(sprite_entity).despawn();
                        break;
                    }
                }

                // Then despawn the ground items entity
                commands.entity(entity).despawn();

                // Close inspect UI and return to player turn
                next_phase.set(TurnPhase::PlayerTurn);
            }
        }
        // If items remain, we stay in InspectingItems and the UI will rebuild
        // (handled by rebuild system)
    }
}

/// Updates the visual highlighting of items in inspect UI when selection changes
pub fn update_inspect_ui_selection_system(
    selection_query: Query<&InspectSelection>,
    mut item_rows_query: Query<(&InspectItemRow, &mut BackgroundColor)>,
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
pub fn inspect_autoscroll_system(
    selection_query: Query<&InspectSelection>,
    mut scroll_area: Query<(&ComputedNode, &mut ScrollPosition), With<ModalScrollArea>>,
    rows: Query<&InspectItemRow>,
) {
    let Ok(selection) = selection_query.single() else {
        return;
    };
    let Ok((node, mut scroll)) = scroll_area.single_mut() else {
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

/// Rebuilds the inspect UI when ground items change (e.g., after pickup)
pub fn rebuild_inspect_ui_system(
    mut commands: Commands,
    player_query: Query<(&crate::components::components::Position, &Inventory), With<Player>>,
    ground_items_query: Query<(&crate::components::components::Position, &GroundItems), Changed<GroundItems>>,
    ui_query: Query<Entity, With<InspectUiRoot>>,
    selection_query: Query<&InspectSelection>,
    capacity: Res<CarryCapacity>,
) {
    // Only rebuild if ground items changed
    if ground_items_query.is_empty() {
        return;
    }

    let Ok((player_pos, inventory)) = player_query.single() else {
        return;
    };

    // Check if there are still items at player's position
    let Some(items) = ground_items_query
        .iter()
        .find(|(pos, _)| pos.x == player_pos.x && pos.y == player_pos.y)
        .map(|(_, ground_items)| &ground_items.items)
    else {
        return;
    };

    // If no items remain, don't rebuild (pickup system will handle closing)
    if items.is_empty() {
        return;
    }

    // Get current selection and adjust if needed
    let current_selection = selection_query.single().map(|s| s.selected_index).unwrap_or(0);
    let adjusted_selection = current_selection.min(items.len().saturating_sub(1));

    // Despawn old UI (if it still exists - it might have been despawned by state transition)
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }

    // Respawn UI with updated item list
    build_inspect_ui(
        &mut commands,
        &items,
        adjusted_selection,
        inventory.total_weight(),
        capacity.normal,
    );
}
