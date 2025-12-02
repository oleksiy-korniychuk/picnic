use bevy::prelude::*;
use crate::components::{
    components::{Player, Position},
    inventory::Inventory,
};
use crate::resources::{
    turn_state::TurnPhase,
    contract_system::{ContractSystem, ContractStatus},
    game_state::GameState,
    turn_state::TurnCounter,
    message_log::MessageLog,
};

// ============================================================================
// ENTER THE ZONE SCREEN
// ============================================================================

/// Marker component for the Enter Zone UI root
#[derive(Component)]
pub struct EnterZoneUiRoot;

/// Spawns the Enter Zone UI when entering EnteringZone phase
pub fn spawn_enter_zone_ui_system(
    mut commands: Commands,
    contract_system: Res<ContractSystem>,
    existing_ui: Query<Entity, With<EnterZoneUiRoot>>,
) {
    // Don't spawn if UI already exists
    if existing_ui.iter().next().is_some() {
        return;
    }

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            EnterZoneUiRoot,
            ZIndex(100),
        ))
        .with_children(|parent| {
            // Modal panel
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(30.0)),
                        row_gap: Val::Px(15.0),
                        min_width: Val::Px(500.0),
                        max_width: Val::Px(700.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Mission Briefing"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.3)),
                    ));

                    // Subtitle
                    parent.spawn((
                        Text::new("Active Contracts:"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));

                    // Contract list
                    for (index, contract) in contract_system.active_contracts.iter().enumerate() {
                        parent.spawn((
                            Text::new(format!("{}. {}", index + 1, contract.description)),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            Node {
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                    }

                    // Separator
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(2.0),
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    ));

                    // Help text
                    parent.spawn((
                        Text::new("E - Accept and Enter the Zone"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.9, 0.6)),
                    ));
                });
        });
}

/// Despawns the Enter Zone UI when exiting EnteringZone phase
pub fn despawn_enter_zone_ui_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<EnterZoneUiRoot>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles E key to close Enter Zone UI and start game
pub fn close_enter_zone_ui_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        next_phase.set(TurnPhase::PlayerTurn);
    }
}

// ============================================================================
// EXIT THE ZONE SCREEN
// ============================================================================

/// Marker component for the Exit Zone UI root
#[derive(Component)]
pub struct ExitZoneUiRoot;

/// Spawns the Exit Zone UI when entering ExitingZone phase
pub fn spawn_exit_zone_ui_system(
    mut commands: Commands,
    mut contract_system: ResMut<ContractSystem>,
    player_query: Query<&Inventory, With<Player>>,
    existing_ui: Query<Entity, With<ExitZoneUiRoot>>,
) {
    // Don't spawn if UI already exists
    if existing_ui.iter().next().is_some() {
        return;
    }

    // Get player inventory and validate contracts
    let Ok(inventory) = player_query.single() else {
        return;
    };

    let contract_statuses = contract_system.validate_contracts(inventory);

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ExitZoneUiRoot,
            ZIndex(100),
        ))
        .with_children(|parent| {
            // Modal panel
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(30.0)),
                        row_gap: Val::Px(15.0),
                        min_width: Val::Px(500.0),
                        max_width: Val::Px(700.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Extraction Point"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.3)),
                    ));

                    // Contract status
                    parent.spawn((
                        Text::new("Contract Status:"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));

                    // Contract list with completion status
                    for status in contract_statuses.iter() {
                        let (marker, color) = if status.completed {
                            ("✓", Color::srgb(0.3, 0.9, 0.3)) // Green checkmark
                        } else {
                            ("✗", Color::srgb(0.9, 0.3, 0.3)) // Red X
                        };

                        parent
                            .spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(10.0),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(marker),
                                    TextFont {
                                        font_size: 22.0,
                                        ..default()
                                    },
                                    TextColor(color),
                                ));

                                parent.spawn((
                                    Text::new(&status.description),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                    }

                    // Separator
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(2.0),
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    ));

                    // Help text
                    parent.spawn((
                        Text::new("E - Exit the Zone"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.9, 0.6)),
                    ));
                });
        });
}

/// Despawns the Exit Zone UI when exiting ExitingZone phase
pub fn despawn_exit_zone_ui_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<ExitZoneUiRoot>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles E key to exit zone and reset game
pub fn close_exit_zone_ui_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        // Transition to Editing which will trigger reset and then back to Running
        next_state.set(GameState::Editing);
    }
}

// ============================================================================
// DEATH SCREEN
// ============================================================================

/// Marker component for the Death UI root
#[derive(Component)]
pub struct DeathUiRoot;

/// Spawns the Death UI when entering PlayerDead phase
pub fn spawn_death_ui_system(
    mut commands: Commands,
    existing_ui: Query<Entity, With<DeathUiRoot>>,
) {
    // Don't spawn if UI already exists
    if existing_ui.iter().next().is_some() {
        return;
    }

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            DeathUiRoot,
            ZIndex(100),
        ))
        .with_children(|parent| {
            // Modal panel
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(40.0)),
                        row_gap: Val::Px(20.0),
                        min_width: Val::Px(500.0),
                        max_width: Val::Px(700.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                    BorderColor(Color::srgb(0.8, 0.2, 0.2)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("DEATH"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.2, 0.2)),
                    ));

                    // Death message
                    parent.spawn((
                        Text::new("Red has met his end in the Zone"),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::vertical(Val::Px(20.0)),
                            ..default()
                        },
                    ));

                    // Separator
                    parent.spawn((
                        Node {
                            width: Val::Percent(80.0),
                            height: Val::Px(2.0),
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    ));

                    // Help text
                    parent.spawn((
                        Text::new("E - Restart with a new Stalker"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.9, 0.6)),
                    ));
                });
        });
}

/// Despawns the Death UI when exiting PlayerDead phase
pub fn despawn_death_ui_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<DeathUiRoot>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles E key to restart after death
pub fn close_death_ui_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        // Transition to Editing which will trigger reset and then back to Running
        next_state.set(GameState::Editing);
    }
}

// ============================================================================
// EXIT DETECTION SYSTEM
// ============================================================================

/// Detects when player steps on an exit tile and transitions to ExitingZone phase
pub fn detect_exit_system(
    player_query: Query<&Position, With<Player>>,
    exit_query: Query<(&Position, &crate::resources::game_grid::EntityType)>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    // Check if player is on an exit tile
    for (exit_pos, entity_type) in exit_query.iter() {
        if matches!(entity_type, crate::resources::game_grid::EntityType::Exit)
            && player_pos.x == exit_pos.x
            && player_pos.y == exit_pos.y
        {
            next_phase.set(TurnPhase::ExitingZone);
            info!("Player reached exit at ({}, {})", exit_pos.x, exit_pos.y);
            return;
        }
    }
}

// ============================================================================
// GAME RESET SYSTEM
// ============================================================================

/// Resource to track if we need to auto-restart the game
#[derive(Resource, Default)]
pub struct AutoRestartFlag {
    pub should_restart: bool,
}

/// System that triggers when entering Editing mode from a death/exit
/// Sets a flag to auto-restart the game
pub fn prepare_restart_system(
    mut auto_restart: ResMut<AutoRestartFlag>,
    mut contract_system: ResMut<ContractSystem>,
    mut turn_counter: ResMut<TurnCounter>,
    mut message_log: ResMut<MessageLog>,
) {
    // Reset game state
    contract_system.reset();
    turn_counter.0 = 0;
    message_log.clear();

    // Set flag to restart
    auto_restart.should_restart = true;
    info!("Game state reset, preparing to restart");
}

/// System that runs in Editing mode and auto-restarts if flag is set
pub fn auto_restart_system(
    mut auto_restart: ResMut<AutoRestartFlag>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if auto_restart.should_restart {
        auto_restart.should_restart = false;
        next_state.set(GameState::Running);
        info!("Auto-restarting game");
    }
}

/// Modifies the player spawn system to set EnteringZone phase instead of PlayerTurn
pub fn set_entering_zone_phase_system(
    mut next_phase: ResMut<NextState<TurnPhase>>,
) {
    next_phase.set(TurnPhase::EnteringZone);
}
