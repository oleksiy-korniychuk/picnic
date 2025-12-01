use bevy::prelude::*;
use crate::components::{
    components::{Player, Position},
    inventory::{Inventory, CarryCapacity},
    item::GroundItems,
};
use crate::resources::{
    turn_state::TurnPhase,
};

/// Marker component for the inventory UI root
#[derive(Component)]
pub struct InventoryUiRoot;

/// Marker component for inventory item list
#[derive(Component)]
pub struct InventoryItemList;

/// Component tracking which item is selected
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

    let current_weight = inventory.total_weight();
    let max_capacity = if gravity_timer.is_some() {
        capacity.in_gravity
    } else {
        capacity.normal
    };
    let is_overweight = current_weight > max_capacity;

    // Create modal UI - similar to inspect UI structure
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), // Semi-transparent overlay
            InventoryUiRoot,
            ZIndex(100), // Ensure it's on top
        ))
        .with_children(|parent| {
            // Modal panel
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(10.0),
                        width: Val::Px(600.0),
                        max_height: Val::Percent(80.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Inventory"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Weight display
                    let weight_text = if is_overweight {
                        format!("Weight: {}/{} (OVERWEIGHT!)", current_weight, max_capacity)
                    } else {
                        format!("Weight: {}/{}", current_weight, max_capacity)
                    };
                    parent.spawn((
                        Text::new(weight_text),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(if is_overweight {
                            Color::srgb(1.0, 0.3, 0.3)
                        } else {
                            Color::srgb(0.7, 0.7, 0.7)
                        }),
                    ));

                    // Item list
                    parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(5.0),
                                overflow: Overflow::scroll_y(),
                                max_height: Val::Px(400.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                            InventoryItemList,
                            InventorySelection { selected_index: 0 },
                        ))
                        .with_children(|parent| {
                            if inventory.is_empty() {
                                parent.spawn((
                                    Text::new("(Empty)"),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                ));
                            } else {
                                for (index, item) in inventory.items.iter().enumerate() {
                                    let value_str = match item.value {
                                        Some(v) => format!("Value: {}", v),
                                        None => "Tool".to_string(),
                                    };
                                    let metal_str = if item.is_metal { " [Metal]" } else { "" };
                                    let item_text = format!(
                                        "{}. {} (Weight: {}, {}){}",
                                        index + 1,
                                        item.name,
                                        item.weight,
                                        value_str,
                                        metal_str
                                    );

                                    let bg_color = if index == 0 {
                                        Color::srgb(0.3, 0.5, 0.3) // Highlighted (green)
                                    } else {
                                        Color::srgb(0.1, 0.1, 0.1) // Normal
                                    };

                                    parent.spawn((
                                        Node {
                                            padding: UiRect::all(Val::Px(5.0)),
                                            ..default()
                                        },
                                        BackgroundColor(bg_color),
                                        InventoryItemRow { index },
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new(item_text),
                                            TextFont {
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        ));
                                    });
                                }
                            }
                        });

                    // Help text
                    parent.spawn((
                        Text::new("W/S to select, D to drop, ESC to close"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });
        });
}

/// Despawns the inventory UI when exiting ViewingInventory phase
pub fn despawn_inventory_ui_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<InventoryUiRoot>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
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

/// Handles W/S key navigation in inventory
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

    // S = down, W = up (consistent with movement)
    if keyboard.just_pressed(KeyCode::KeyS) {
        if selection.selected_index < max_index {
            selection.selected_index += 1;
        }
    } else if keyboard.just_pressed(KeyCode::KeyW) {
        if selection.selected_index > 0 {
            selection.selected_index -= 1;
        }
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

/// Auto-scrolls the inventory list to keep selected item visible
pub fn auto_scroll_inventory_system(
    selection_query: Query<&InventorySelection, Changed<InventorySelection>>,
    mut scroll_query: Query<&mut ScrollPosition, With<InventoryItemList>>,
) {
    // Only update scroll when selection changes
    let Ok(selection) = selection_query.single() else {
        return;
    };

    let Ok(mut scroll_pos) = scroll_query.single_mut() else {
        return;
    };

    // Layout measurements:
    // - Container: max_height 400px with 10px padding top/bottom
    // - Each item: 5px pad + ~22px text + 5px pad + 5px gap = 37px
    // - Measured: 10 full items + 1/3 of 11th visible in 380px viewport = 37px per item
    const ITEM_HEIGHT: f32 = 37.0;
    const VIEWPORT_HEIGHT: f32 = 380.0; // 400px container - 20px padding
    const SCROLL_MARGIN: f32 = 20.0; // Trigger scrolling before item reaches edge

    // Calculate item bounds in content space
    let item_top = (selection.selected_index as f32) * ITEM_HEIGHT;
    let item_bottom = item_top + ITEM_HEIGHT;

    // Current visible range
    let current_scroll = scroll_pos.offset_y;
    let viewport_top = current_scroll;
    let viewport_bottom = current_scroll + VIEWPORT_HEIGHT;

    // Determine if we need to scroll
    if item_top < viewport_top + SCROLL_MARGIN {
        // Item is approaching top of viewport - scroll up
        // Position item at SCROLL_MARGIN from top (but not negative)
        scroll_pos.offset_y = (item_top - SCROLL_MARGIN).max(0.0);
    } else if item_bottom > viewport_bottom - SCROLL_MARGIN {
        // Item is approaching bottom of viewport - scroll down
        // Position item at SCROLL_MARGIN from bottom
        scroll_pos.offset_y = item_bottom + SCROLL_MARGIN - VIEWPORT_HEIGHT;
        // Note: Bevy will automatically clamp to max scroll based on content height
    }
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

    // Despawn old UI
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Rebuild UI with updated inventory
    let current_weight = inventory.total_weight();
    let max_capacity = if gravity_timer.is_some() {
        capacity.in_gravity
    } else {
        capacity.normal
    };
    let is_overweight = current_weight > max_capacity;

    // Clamp selection to valid range
    let max_index = if inventory.is_empty() {
        0
    } else {
        inventory.count() - 1
    };
    let clamped_selection = selected_index.min(max_index);

    // Spawn new UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            InventoryUiRoot,
            ZIndex(100),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(10.0),
                        width: Val::Px(600.0),
                        max_height: Val::Percent(80.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Inventory"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    let weight_text = if is_overweight {
                        format!("Weight: {}/{} (OVERWEIGHT!)", current_weight, max_capacity)
                    } else {
                        format!("Weight: {}/{}", current_weight, max_capacity)
                    };
                    parent.spawn((
                        Text::new(weight_text),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(if is_overweight {
                            Color::srgb(1.0, 0.3, 0.3)
                        } else {
                            Color::srgb(0.7, 0.7, 0.7)
                        }),
                    ));

                    parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(5.0),
                                overflow: Overflow::scroll_y(),
                                max_height: Val::Px(400.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                            InventoryItemList,
                            InventorySelection { selected_index: clamped_selection },
                        ))
                        .with_children(|parent| {
                            if inventory.is_empty() {
                                parent.spawn((
                                    Text::new("(Empty)"),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                ));
                            } else {
                                for (index, item) in inventory.items.iter().enumerate() {
                                    let value_str = match item.value {
                                        Some(v) => format!("Value: {}", v),
                                        None => "Tool".to_string(),
                                    };
                                    let metal_str = if item.is_metal { " [Metal]" } else { "" };
                                    let item_text = format!(
                                        "{}. {} (Weight: {}, {}){}",
                                        index + 1,
                                        item.name,
                                        item.weight,
                                        value_str,
                                        metal_str
                                    );

                                    let bg_color = if index == clamped_selection {
                                        Color::srgb(0.3, 0.5, 0.3)
                                    } else {
                                        Color::srgb(0.1, 0.1, 0.1)
                                    };

                                    parent.spawn((
                                        Node {
                                            padding: UiRect::all(Val::Px(5.0)),
                                            ..default()
                                        },
                                        BackgroundColor(bg_color),
                                        InventoryItemRow { index },
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new(item_text),
                                            TextFont {
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        ));
                                    });
                                }
                            }
                        });

                    parent.spawn((
                        Text::new("W/S to select, D to drop, ESC to close"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });
        });
}
