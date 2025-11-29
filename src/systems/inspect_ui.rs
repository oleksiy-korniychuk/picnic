use bevy::prelude::*;
use crate::components::{
    components::Player,
    item::GroundItems,
    inventory::{Inventory, CarryCapacity},
};
use crate::resources::{
    turn_state::TurnPhase,
    message_log::MessageLog,
};
use crate::systems::ground_items::GroundItemSprite;

/// Marker component for the inspect UI root
#[derive(Component)]
pub struct InspectUiRoot;

/// Marker component for the item list container
#[derive(Component)]
pub struct InspectItemList;

/// Marker component for individual item rows in inspect UI
#[derive(Component)]
pub struct InspectItemRow {
    pub index: usize,
}

/// Component tracking which item is selected for pickup
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

    let items = ground_items_query
        .iter()
        .find(|(pos, _)| pos.x == player_pos.x && pos.y == player_pos.y)
        .map(|(_, ground_items)| &ground_items.items);

    let Some(items) = items else {
        return;
    };

    let current_weight = inventory.total_weight();

    // Create modal UI
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
            InspectUiRoot,
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
                        min_width: Val::Px(400.0),
                        max_height: Val::Percent(80.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Items on Ground"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Current weight display
                    parent.spawn((
                        Text::new(format!("Current Weight: {}/{}", current_weight, capacity.normal)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
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
                            InspectItemList,
                            InspectSelection { selected_index: 0 },
                        ))
                        .with_children(|parent| {
                            for (index, item) in items.iter().enumerate() {
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
                                    InspectItemRow { index },
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
                        });

                    // Help text
                    parent.spawn((
                        Text::new("W/S to select, E to pickup, ESC to close"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });
        });
}

/// Despawns the inspect UI when exiting InspectingItems phase
pub fn despawn_inspect_ui_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<InspectUiRoot>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles ESC key to close inspect UI (when in InspectingItems phase)
pub fn close_inspect_ui_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_phase.set(TurnPhase::PlayerTurn);
    }
}

/// Handles W/S key navigation in inspect UI
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
    let items = ground_items_query
        .iter()
        .find(|(pos, _)| pos.x == player_pos.x && pos.y == player_pos.y)
        .map(|(_, ground_items)| &ground_items.items);

    let Some(items) = items else {
        return;
    };

    // If no items remain, don't rebuild (pickup system will handle closing)
    if items.is_empty() {
        return;
    }

    // Get current selection and adjust if needed
    let current_selection = selection_query.single().map(|s| s.selected_index).unwrap_or(0);
    let adjusted_selection = current_selection.min(items.len().saturating_sub(1));

    // Despawn old UI
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let current_weight = inventory.total_weight();

    // Respawn UI with updated item list
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
            InspectUiRoot,
            ZIndex(100),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(10.0),
                        min_width: Val::Px(400.0),
                        max_height: Val::Percent(80.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Items on Ground"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    parent.spawn((
                        Text::new(format!("Current Weight: {}/{}", current_weight, capacity.normal)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
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
                            InspectItemList,
                            InspectSelection { selected_index: adjusted_selection },
                        ))
                        .with_children(|parent| {
                            for (index, item) in items.iter().enumerate() {
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

                                let bg_color = if index == adjusted_selection {
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
                                    InspectItemRow { index },
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
                        });

                    parent.spawn((
                        Text::new("W/S to select, E to pickup, ESC to close"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });
        });
}
