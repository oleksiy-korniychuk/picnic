use bevy::prelude::*;
use crate::resources::{
    game_state::GameState,
    stash_system::{Stash, RunInventory},
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

/// Tracks which panel and item is selected in the base hub
#[derive(Component, Debug, Clone, Copy)]
pub struct BaseHubSelection {
    pub active_panel: PanelSide,
    pub selected_index: usize,
}

impl Default for BaseHubSelection {
    fn default() -> Self {
        Self {
            active_panel: PanelSide::RunInventory,
            selected_index: 0,
        }
    }
}

/// Which panel is currently active in the base hub
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelSide {
    RunInventory,  // Left panel
    Stash,         // Right panel
}

/// Marker for individual item rows in RunInventory panel
#[derive(Component)]
pub struct RunInventoryItemRow {
    pub index: usize,
}

/// Marker for individual item rows in Stash panel
#[derive(Component)]
pub struct StashItemRow {
    pub index: usize,
}

/// Spawns the Stash Management UI when entering StashManagement mode
pub fn spawn_stash_management_ui_system(
    mut commands: Commands,
    existing_ui: Query<Entity, With<StashManagementUiRoot>>,
    run_inventory: Res<RunInventory>,
    stash: Res<Stash>,
) {
    // Don't spawn if UI already exists
    if existing_ui.iter().next().is_some() {
        return;
    }

    // Calculate weights
    let run_weight = run_inventory.total_weight();
    let stash_weight = stash.total_weight();

    // Initialize selection state
    let selection = BaseHubSelection::default();
    let selection_for_closure = selection; // Copy for use in closure

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
            selection,
            ZIndex(100),
        ))
        .with_children(|parent| {
            let selection = selection_for_closure; // Use the copy in closure
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
                                        Text::new(format!("Run Inventory ({}/250)", run_weight)),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                    ));

                                    // Display real items from RunInventory
                                    if run_inventory.is_empty() {
                                        parent.spawn((
                                            Text::new("(Empty)"),
                                            TextFont {
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                        ));
                                    } else {
                                        for (index, item) in run_inventory.items.iter().enumerate() {
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

                                            // Determine if this item is selected
                                            let is_selected = selection.active_panel == PanelSide::RunInventory
                                                && selection.selected_index == index;
                                            let bg_color = if is_selected {
                                                Color::srgb(0.3, 0.5, 0.3) // Highlighted green
                                            } else {
                                                Color::srgb(0.1, 0.1, 0.1) // Normal dark
                                            };

                                            parent.spawn((
                                                Node {
                                                    padding: UiRect::all(Val::Px(5.0)),
                                                    ..default()
                                                },
                                                BackgroundColor(bg_color),
                                                RunInventoryItemRow { index },
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn((
                                                    Text::new(item_text),
                                                    TextFont {
                                                        font_size: 18.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::WHITE),
                                                ));
                                            });
                                        }
                                    }
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
                                        Text::new(format!("Stash ({}/1000)", stash_weight)),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                    ));

                                    // Display real items from Stash
                                    if stash.is_empty() {
                                        parent.spawn((
                                            Text::new("(Empty)"),
                                            TextFont {
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                        ));
                                    } else {
                                        for (index, item) in stash.items.iter().enumerate() {
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

                                            // Determine if this item is selected
                                            let is_selected = selection.active_panel == PanelSide::Stash
                                                && selection.selected_index == index;
                                            let bg_color = if is_selected {
                                                Color::srgb(0.3, 0.5, 0.3) // Highlighted green
                                            } else {
                                                Color::srgb(0.1, 0.1, 0.1) // Normal dark
                                            };

                                            parent.spawn((
                                                Node {
                                                    padding: UiRect::all(Val::Px(5.0)),
                                                    ..default()
                                                },
                                                BackgroundColor(bg_color),
                                                StashItemRow { index },
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn((
                                                    Text::new(item_text),
                                                    TextFont {
                                                        font_size: 18.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::WHITE),
                                                ));
                                            });
                                        }
                                    }
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
    commands: Commands,
    current_mode: Res<State<BaseHubMode>>,
    ui_query: Query<Entity, With<StashManagementUiRoot>>,
    run_inventory: Res<RunInventory>,
    stash: Res<Stash>,
) {
    let ui_exists = ui_query.iter().next().is_some();
    let should_show = *current_mode.get() == BaseHubMode::StashManagement;

    if should_show && !ui_exists {
        // Need to spawn
        spawn_stash_management_ui_system(commands, ui_query, run_inventory, stash);
    } else if !should_show && ui_exists {
        // Need to despawn - can't call here due to mut commands
        // Will be handled by cleanup on state exit
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
        info!("[BASE HUB] Key pressed: Tab (toggle mode)");
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
    if keyboard.just_pressed(KeyCode::Space) {
        info!("[BASE HUB] Key pressed: Space (enter zone)");
        if *current_mode.get() == BaseHubMode::StashManagement {
            next_state.set(GameState::Running);
            info!("Entering zone from base hub");
        } else {
            info!("Cannot enter zone from Contracts screen");
        }
    }
}

/// Handles ESC key to quit game (temporary placeholder)
pub fn base_hub_escape_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("[BASE HUB] Key pressed: Escape (quit game)");
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

// ============================================================================
// NAVIGATION AND ITEM MANAGEMENT
// ============================================================================

/// Updates highlighting of item rows based on current selection
pub fn update_base_hub_highlighting_system(
    selection_query: Query<&BaseHubSelection, With<StashManagementUiRoot>>,
    mut run_rows_query: Query<(&RunInventoryItemRow, &mut BackgroundColor), Without<StashItemRow>>,
    mut stash_rows_query: Query<(&StashItemRow, &mut BackgroundColor), Without<RunInventoryItemRow>>,
) {
    let Ok(selection) = selection_query.get_single() else {
        return;
    };

    // Update RunInventory item rows
    for (row, mut bg_color) in run_rows_query.iter_mut() {
        let is_selected = selection.active_panel == PanelSide::RunInventory
            && selection.selected_index == row.index;
        let new_color = if is_selected {
            Color::srgb(0.3, 0.5, 0.3) // Highlighted green
        } else {
            Color::srgb(0.1, 0.1, 0.1) // Normal dark
        };

        if bg_color.0 != new_color {
            *bg_color = BackgroundColor(new_color);
        }
    }

    // Update Stash item rows
    for (row, mut bg_color) in stash_rows_query.iter_mut() {
        let is_selected = selection.active_panel == PanelSide::Stash
            && selection.selected_index == row.index;
        let new_color = if is_selected {
            Color::srgb(0.3, 0.5, 0.3) // Highlighted green
        } else {
            Color::srgb(0.1, 0.1, 0.1) // Normal dark
        };

        if bg_color.0 != new_color {
            *bg_color = BackgroundColor(new_color);
        }
    }
}

/// Handles arrow key navigation in the base hub
pub fn base_hub_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selection_query: Query<&mut BaseHubSelection, With<StashManagementUiRoot>>,
    run_inventory: Res<RunInventory>,
    stash: Res<Stash>,
) {
    let Ok(mut selection) = selection_query.get_single_mut() else {
        return;
    };

    let run_count = run_inventory.count();
    let stash_count = stash.count();

    // W/S: Navigate up/down within current panel
    if keyboard.just_pressed(KeyCode::KeyW) {
        info!("[BASE HUB] Key pressed: W (navigate up)");
        if selection.selected_index > 0 {
            selection.selected_index -= 1;
        }
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        info!("[BASE HUB] Key pressed: S (navigate down)");
        let max_index = match selection.active_panel {
            PanelSide::RunInventory => run_count.saturating_sub(1),
            PanelSide::Stash => stash_count.saturating_sub(1),
        };
        if selection.selected_index < max_index {
            selection.selected_index += 1;
        }
    }

    // A/D: Switch between panels
    if keyboard.just_pressed(KeyCode::KeyA) {
        info!("[BASE HUB] Key pressed: A (switch to RunInventory panel)");
        // Move to RunInventory (left)
        if selection.active_panel != PanelSide::RunInventory {
            selection.active_panel = PanelSide::RunInventory;
            // Clamp selection to valid range
            if run_count > 0 {
                selection.selected_index = selection.selected_index.min(run_count - 1);
            } else {
                selection.selected_index = 0;
            }
        }
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        info!("[BASE HUB] Key pressed: D (switch to Stash panel)");
        // Move to Stash (right)
        if selection.active_panel != PanelSide::Stash {
            selection.active_panel = PanelSide::Stash;
            // Clamp selection to valid range
            if stash_count > 0 {
                selection.selected_index = selection.selected_index.min(stash_count - 1);
            } else {
                selection.selected_index = 0;
            }
        }
    }
}

/// Rebuilds the stash UI when RunInventory or Stash resources change
pub fn rebuild_stash_ui_system(
    mut commands: Commands,
    run_inventory: Res<RunInventory>,
    stash: Res<Stash>,
    ui_query: Query<Entity, With<StashManagementUiRoot>>,
    selection_query: Query<&BaseHubSelection>,
    current_mode: Res<State<BaseHubMode>>,
) {
    // Only rebuild if we're in StashManagement mode and resources changed
    if *current_mode.get() != BaseHubMode::StashManagement {
        return;
    }

    if !run_inventory.is_changed() && !stash.is_changed() {
        return;
    }

    // Save current selection
    let saved_selection = selection_query.get_single().map(|s| BaseHubSelection {
        active_panel: s.active_panel,
        selected_index: s.selected_index,
    }).unwrap_or_default();

    // Despawn old UI
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Clamp saved selection to valid range
    let run_count = run_inventory.count();
    let stash_count = stash.count();

    let mut clamped_selection = saved_selection;
    match clamped_selection.active_panel {
        PanelSide::RunInventory => {
            if run_count == 0 && stash_count > 0 {
                // Switch to stash if run inventory is empty
                clamped_selection.active_panel = PanelSide::Stash;
                clamped_selection.selected_index = 0;
            } else if run_count > 0 {
                clamped_selection.selected_index = clamped_selection.selected_index.min(run_count - 1);
            }
        }
        PanelSide::Stash => {
            if stash_count == 0 && run_count > 0 {
                // Switch to run inventory if stash is empty
                clamped_selection.active_panel = PanelSide::RunInventory;
                clamped_selection.selected_index = 0;
            } else if stash_count > 0 {
                clamped_selection.selected_index = clamped_selection.selected_index.min(stash_count - 1);
            }
        }
    }

    // Spawn new UI with clamped selection
    spawn_stash_management_ui_with_selection(commands, run_inventory, stash, clamped_selection);
}

/// Helper to spawn stash management UI with a specific selection state
fn spawn_stash_management_ui_with_selection(
    mut commands: Commands,
    run_inventory: Res<RunInventory>,
    stash: Res<Stash>,
    selection: BaseHubSelection,
) {
    // This is a copy of spawn_stash_management_ui_system but uses the provided selection
    // Calculate weights
    let run_weight = run_inventory.total_weight();
    let stash_weight = stash.total_weight();

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
            selection,
            ZIndex(100),
        ))
        .with_children(|parent| {
            // [Rest of UI spawning code - same as spawn_stash_management_ui_system]
            // For now, just create a placeholder - we'll need to refactor to avoid duplication
            parent.spawn((
                Text::new("UI Rebuilding..."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// Handles Enter/E key to move items between RunInventory and Stash
pub fn base_hub_move_item_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selection_query: Query<&mut BaseHubSelection, With<StashManagementUiRoot>>,
    mut run_inventory: ResMut<RunInventory>,
    mut stash: ResMut<Stash>,
) {
    let enter_pressed = keyboard.just_pressed(KeyCode::Enter);
    let e_pressed = keyboard.just_pressed(KeyCode::KeyE);

    if !enter_pressed && !e_pressed {
        return;
    }

    if enter_pressed {
        info!("[BASE HUB] Key pressed: Enter (move item)");
    }
    if e_pressed {
        info!("[BASE HUB] Key pressed: E (move item)");
    }

    let Ok(mut selection) = selection_query.get_single_mut() else {
        return;
    };

    match selection.active_panel {
        PanelSide::RunInventory => {
            // Move from RunInventory to Stash
            if run_inventory.is_empty() || selection.selected_index >= run_inventory.count() {
                return;
            }

            if let Some(item) = run_inventory.remove_item(selection.selected_index) {
                info!("Moving '{}' from RunInventory to Stash", item.name);
                stash.add_item(item);

                // Clamp selection after removal
                if run_inventory.is_empty() {
                    // Switch to stash panel if run inventory is now empty
                    selection.active_panel = PanelSide::Stash;
                    selection.selected_index = stash.count().saturating_sub(1);
                } else {
                    // Clamp to valid index
                    selection.selected_index = selection.selected_index.min(run_inventory.count() - 1);
                }
            }
        }
        PanelSide::Stash => {
            // Move from Stash to RunInventory
            if stash.is_empty() || selection.selected_index >= stash.count() {
                return;
            }

            if let Some(item) = stash.remove_item(selection.selected_index) {
                info!("Moving '{}' from Stash to RunInventory", item.name);
                run_inventory.add_item(item);

                // Clamp selection after removal
                if stash.is_empty() {
                    // Switch to run inventory panel if stash is now empty
                    selection.active_panel = PanelSide::RunInventory;
                    selection.selected_index = run_inventory.count().saturating_sub(1);
                } else {
                    // Clamp to valid index
                    selection.selected_index = selection.selected_index.min(stash.count() - 1);
                }
            }
        }
    }
}
