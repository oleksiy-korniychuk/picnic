use bevy::prelude::*;
use crate::resources::{
    game_state::GameState,
};

// ============================================================================
// BASE HUB MODE STATE
// ============================================================================

/// Tracks which screen is active in the base hub
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseHubMode {
    #[default]
    StashManagement,
    Contracts,
}

/// Resets base hub mode to StashManagement when entering the base hub
pub fn reset_base_hub_mode_system(
    mut next_mode: ResMut<NextState<BaseHubMode>>,
) {
    next_mode.set(BaseHubMode::StashManagement);
    info!("Reset base hub mode to StashManagement");
}

// ============================================================================
// STASH MANAGEMENT SCREEN (Visual Structure Only)
// ============================================================================

/// Marker component for the Stash Management UI root
#[derive(Component)]
pub struct StashManagementUiRoot;

/// Spawns the Stash Management UI when entering StashManagement mode
pub fn spawn_stash_management_ui_system(
    mut commands: Commands,
    existing_ui: Query<Entity, With<StashManagementUiRoot>>,
) {
    // Don't spawn if UI already exists
    if existing_ui.iter().next().is_some() {
        return;
    }

    // Create full-screen modal UI
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
            StashManagementUiRoot,
            ZIndex(100),
        ))
        .with_children(|parent| {
            // Main panel container
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(40.0)),
                        row_gap: Val::Px(20.0),
                        width: Val::Px(900.0),
                        max_height: Val::Percent(90.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Base Hub - Stash Management"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.3)),
                    ));

                    // Money display
                    parent.spawn((
                        Text::new("Money: 0 Rubles"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.3, 0.9, 0.3)),
                    ));

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

                    // Two-panel layout (Run Inventory | Stash)
                    parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(20.0),
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            // Left panel: Run Inventory
                            parent
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Column,
                                        row_gap: Val::Px(10.0),
                                        width: Val::Percent(50.0),
                                        padding: UiRect::all(Val::Px(15.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                                    BorderColor(Color::srgb(0.3, 0.5, 0.3)),
                                ))
                                .with_children(|parent| {
                                    // Panel header
                                    parent.spawn((
                                        Text::new("Run Inventory (0/250)"),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                    ));

                                    // Placeholder items
                                    parent.spawn((
                                        Text::new("1. Bolt x10 (Weight: 10, Value: 10)"),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));

                                    parent.spawn((
                                        Text::new("2. Metal Detector (Weight: 50, Tool)"),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });

                            // Right panel: Stash
                            parent
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Column,
                                        row_gap: Val::Px(10.0),
                                        width: Val::Percent(50.0),
                                        padding: UiRect::all(Val::Px(15.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                                ))
                                .with_children(|parent| {
                                    // Panel header
                                    parent.spawn((
                                        Text::new("Stash (0/1000)"),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                    ));

                                    // Empty message
                                    parent.spawn((
                                        Text::new("(Empty)"),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                    ));
                                });
                        });

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
                        Text::new("Tab - Contracts | Space - Enter Zone | ESC - Quit Game"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });
        });
}

/// Despawns the Stash Management UI when exiting StashManagement mode
pub fn despawn_stash_management_ui_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<StashManagementUiRoot>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles spawning/despawning stash UI based on mode changes
pub fn handle_stash_ui_spawn_system(
    mut commands: Commands,
    current_mode: Res<State<BaseHubMode>>,
    ui_query: Query<Entity, With<StashManagementUiRoot>>,
) {
    let ui_exists = ui_query.iter().next().is_some();
    let should_show = *current_mode.get() == BaseHubMode::StashManagement;

    if should_show && !ui_exists {
        // Need to spawn
        spawn_stash_management_ui_system(commands, ui_query);
    } else if !should_show && ui_exists {
        // Need to despawn
        despawn_stash_management_ui_system(commands, ui_query);
    }
}

// ============================================================================
// CONTRACTS SCREEN (Visual Structure Only)
// ============================================================================

/// Marker component for the Contracts UI root
#[derive(Component)]
pub struct ContractsUiRoot;

/// Spawns the Contracts UI when entering Contracts mode
pub fn spawn_contracts_ui_system(
    mut commands: Commands,
    existing_ui: Query<Entity, With<ContractsUiRoot>>,
) {
    // Don't spawn if UI already exists
    if existing_ui.iter().next().is_some() {
        return;
    }

    // Create full-screen modal UI
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
            ContractsUiRoot,
            ZIndex(100),
        ))
        .with_children(|parent| {
            // Main panel container
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(40.0)),
                        row_gap: Val::Px(20.0),
                        width: Val::Px(700.0),
                        max_height: Val::Percent(90.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    BorderColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Contracts"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.3)),
                    ));

                    // Active Contracts section
                    parent.spawn((
                        Text::new("Active Contracts (0/3):"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));

                    // Placeholder: No active contracts
                    parent.spawn((
                        Text::new("(None)"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                        Node {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                    ));

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

                    // Available Contracts section
                    parent.spawn((
                        Text::new("Available Contracts:"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));

                    // Placeholder contracts
                    for i in 1..=5 {
                        parent.spawn((
                            Text::new(format!("{}. [Placeholder Contract] - Reward: 100 Rubles", i)),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            Node {
                                padding: UiRect::all(Val::Px(5.0)),
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
                        Text::new("Tab - Stash Management | E - Select/Turn In | ESC - Quit Game"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });
        });
}

/// Despawns the Contracts UI when exiting Contracts mode
pub fn despawn_contracts_ui_system(
    mut commands: Commands,
    ui_query: Query<Entity, With<ContractsUiRoot>>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles spawning/despawning contracts UI based on mode changes
pub fn handle_contracts_ui_spawn_system(
    mut commands: Commands,
    current_mode: Res<State<BaseHubMode>>,
    ui_query: Query<Entity, With<ContractsUiRoot>>,
) {
    let ui_exists = ui_query.iter().next().is_some();
    let should_show = *current_mode.get() == BaseHubMode::Contracts;

    if should_show && !ui_exists {
        // Need to spawn
        spawn_contracts_ui_system(commands, ui_query);
    } else if !should_show && ui_exists {
        // Need to despawn
        despawn_contracts_ui_system(commands, ui_query);
    }
}

// ============================================================================
// INPUT HANDLERS
// ============================================================================

/// Handles Tab key to toggle between Stash Management and Contracts screens
pub fn toggle_base_hub_mode_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_mode: Res<State<BaseHubMode>>,
    mut next_mode: ResMut<NextState<BaseHubMode>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        let new_mode = match current_mode.get() {
            BaseHubMode::StashManagement => BaseHubMode::Contracts,
            BaseHubMode::Contracts => BaseHubMode::StashManagement,
        };
        next_mode.set(new_mode);
        info!("Switching to {:?} mode", new_mode);
    }
}

/// Handles Space key to enter the zone from base hub
pub fn enter_zone_from_base_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_mode: Res<State<BaseHubMode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Only allow entering zone from Stash Management screen
    if *current_mode.get() == BaseHubMode::StashManagement && keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Running);
        info!("Entering zone from base hub");
    }
}

/// Handles ESC key to quit game (temporary placeholder)
pub fn base_hub_escape_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Quitting game from base hub");
        exit.send(AppExit::Success);
    }
}

/// Cleanup system to despawn all base hub UIs when exiting the base hub state
pub fn cleanup_base_hub_ui_system(
    mut commands: Commands,
    stash_ui: Query<Entity, With<StashManagementUiRoot>>,
    contracts_ui: Query<Entity, With<ContractsUiRoot>>,
) {
    // Despawn stash UI if it exists
    for entity in stash_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
    // Despawn contracts UI if it exists
    for entity in contracts_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("Cleaned up base hub UI");
}
