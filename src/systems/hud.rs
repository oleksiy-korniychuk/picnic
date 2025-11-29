use bevy::prelude::*;
use crate::components::components::*;
use crate::resources::{
    turn_state::TurnCounter,
    message_log::MessageLog,
};

/// Spawns the game HUD when entering Running mode
pub fn spawn_game_hud_system(
    mut commands: Commands,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            GameHudRoot,
        ))
        .with_children(|parent| {
            // Stats bar (turn counter + weight)
            parent.spawn(Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect::bottom(Val::Px(5.0)),
                ..default()
            }).with_children(|stats_bar| {
                // Turn counter (left)
                stats_bar.spawn((
                    Text::new("Turn: 0"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TurnCounterText,
                ));

                // Weight display (right)
                stats_bar.spawn((
                    Text::new("Weight: 0/250"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    WeightText,
                ));
            });

            // Message log container
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(2.0),
                    ..default()
                },
                MessageLogContainer,
            )).with_children(|log_container| {
                // Create 5 message line entities (0=oldest visible, 4=newest)
                for index in 0..5 {
                    log_container.spawn((
                        Text::new(""),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6 + (index as f32 * 0.1))),
                        MessageLogText { index },
                    ));
                }
            });
        });

    info!("Game HUD spawned");
}

/// Despawns the game HUD when exiting Running mode
pub fn despawn_game_hud_system(
    mut commands: Commands,
    hud_query: Query<Entity, With<GameHudRoot>>,
) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn();
    }
    info!("Game HUD despawned");
}

/// Updates the turn counter display
pub fn update_turn_counter_system(
    turn_counter: Res<TurnCounter>,
    mut query: Query<&mut Text, With<TurnCounterText>>,
) {
    if !turn_counter.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        **text = format!("Turn: {}", turn_counter.0);
    }
}

/// Updates the weight display based on player's inventory
pub fn update_weight_display_system(
    mut query: Query<&mut Text, With<WeightText>>,
    player_query: Query<(&crate::components::inventory::Inventory, Option<&crate::components::components::GravitationalAnomalyTimer>), With<crate::components::components::Player>>,
    capacity: Res<crate::components::inventory::CarryCapacity>,
) {
    let Ok((inventory, gravity_timer)) = player_query.single() else {
        return;
    };

    let current_weight = inventory.total_weight();
    let max_capacity = if gravity_timer.is_some() {
        capacity.in_gravity
    } else {
        capacity.normal
    };

    for mut text in query.iter_mut() {
        // Show red text if over capacity
        if current_weight > max_capacity {
            **text = format!("Weight: {}/{} (OVERWEIGHT!)", current_weight, max_capacity);
        } else {
            **text = format!("Weight: {}/{}", current_weight, max_capacity);
        }
    }
}

/// Updates the message log display with recent messages
pub fn update_message_log_system(
    message_log: Res<MessageLog>,
    mut query: Query<(&mut Text, &MessageLogText)>,
) {
    if !message_log.is_changed() {
        return;
    }

    // Collect messages into a Vec for indexing
    let messages: Vec<&String> = message_log.get_messages().collect();
    let message_count = messages.len();

    // Update each message line
    for (mut text, log_text) in query.iter_mut() {
        // Calculate which message to show
        // We want to fill from bottom up: index 4 = newest, index 3 = second newest, etc.
        let message_index = if message_count > log_text.index {
            // Show older messages first (message 0 at index 0, message 1 at index 1, etc.)
            Some(log_text.index)
        } else {
            None
        };

        if let Some(idx) = message_index {
            if idx < message_count {
                **text = messages[idx].clone();
            } else {
                **text = String::new();
            }
        } else {
            **text = String::new();
        }
    }
}
