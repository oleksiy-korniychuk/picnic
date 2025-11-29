use bevy::prelude::*;
use crate::components::{
    components::Player,
    item::GroundItems,
};
use crate::resources::turn_state::TurnPhase;

/// Marker component for the inspect UI root
#[derive(Component)]
pub struct InspectUiRoot;

/// Marker component for the item list container
#[derive(Component)]
pub struct InspectItemList;

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
    player_query: Query<&crate::components::components::Position, With<Player>>,
    ground_items_query: Query<(&crate::components::components::Position, &GroundItems)>,
    existing_ui: Query<Entity, With<InspectUiRoot>>,
) {
    // Don't spawn if UI already exists
    if existing_ui.iter().next().is_some() {
        return;
    }

    // Find items at player's position
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let items = ground_items_query
        .iter()
        .find(|(pos, _)| pos.x == player_pos.x && pos.y == player_pos.y)
        .map(|(_, ground_items)| &ground_items.items);

    let Some(items) = items else {
        return;
    };

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

                    // Item list
                    parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(5.0),
                                overflow: Overflow::scroll_y(),
                                max_height: Val::Px(400.0),
                                ..default()
                            },
                            InspectItemList,
                        ))
                        .with_children(|parent| {
                            for (index, item) in items.iter().enumerate() {
                                let value_str = match item.value {
                                    Some(v) => format!("Value: {}", v),
                                    None => "Tool".to_string(),
                                };
                                let item_text = format!(
                                    "{}. {} (Weight: {}, {})",
                                    index + 1,
                                    item.name,
                                    item.weight,
                                    value_str
                                );

                                parent.spawn((
                                    Text::new(item_text),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ));
                            }
                        });

                    // Help text
                    parent.spawn((
                        Text::new("Press ESC to close"),
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
        commands.entity(entity).despawn();
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
