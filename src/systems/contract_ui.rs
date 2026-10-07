use bevy::prelude::*;
use crate::components::{
    components::{Player, Position},
    inventory::Inventory,
};
use crate::systems::ui_kit::*;
use crate::resources::{
    turn_state::TurnPhase,
    contract_system::ContractSystem,
    game_state::GameState,
    turn_state::TurnCounter,
    message_log::MessageLog,
    stash_system::{Stash, RunInventory},
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

    spawn_modal(
        &mut commands,
        EnterZoneUiRoot,
        "Mission Briefing",
        520.0,
        380.0,
        60.0,
        true,
        |content| {
            modal_text(content, "Active Contracts:", FONT_SUB, COL_TEXT);
            if contract_system.active_contracts.is_empty() {
                modal_text(content, "No active contracts - free run.", FONT_BODY, COL_TEXT);
            }
            for (index, contract) in contract_system.active_contracts.iter().enumerate() {
                modal_text(
                    content,
                    format!("{}. {}", index + 1, contract.description),
                    FONT_BODY,
                    Color::WHITE,
                );
            }
        },
        &[hint("E", "Accept and Enter"), hint("Esc", "Skip")],
    );
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
    // E accepts the briefing; ESC skips it (the player is already in the Zone).
    if keyboard.just_pressed(KeyCode::KeyE) || keyboard.just_pressed(KeyCode::Escape) {
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

    spawn_modal(
        &mut commands,
        ExitZoneUiRoot,
        "Extraction Point",
        520.0,
        380.0,
        60.0,
        true,
        |content| {
            modal_text(content, "Contract Status:", FONT_SUB, COL_TEXT);
            if contract_statuses.is_empty() {
                modal_text(content, "No active contracts.", FONT_BODY, COL_TEXT);
            }
            for status in contract_statuses.iter() {
                let (marker, color) = if status.completed {
                    ("[COMPLETE]", COL_SUCCESS)
                } else {
                    ("[FAILED]", COL_FAIL)
                };
                content
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    })
                    .with_children(|row| {
                        modal_text(row, marker, FONT_SUB, color);
                        modal_text(row, status.description.clone(), FONT_BODY, Color::WHITE);
                    });
            }
        },
        &[hint("E", "Return to Base"), hint("Esc", "Stay in the Zone")],
    );
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

/// Handles E key to exit zone and return to base hub
pub fn close_exit_zone_ui_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
    player_query: Query<&Inventory, With<Player>>,
    mut run_inventory: ResMut<RunInventory>,
) {
    // ESC cancels the extraction and stays in the Zone (nothing is saved).
    if keyboard.just_pressed(KeyCode::Escape) {
        next_phase.set(TurnPhase::PlayerTurn);
        info!("Extraction cancelled - staying in the Zone");
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyE) {
        // Save player's inventory to RunInventory before returning to base
        if let Ok(player_inventory) = player_query.get_single() {
            run_inventory.clear();
            for item in player_inventory.items.iter() {
                run_inventory.add_item(item.clone());
            }
            info!("Saved {} items from player to RunInventory (weight: {})",
                  run_inventory.count(),
                  run_inventory.total_weight());
        }

        // Transition to InBaseHub (Stash Management screen)
        next_state.set(GameState::InBaseHub);
        // Reset turn phase to PlayerTurn (will be overridden when re-entering zone)
        next_phase.set(TurnPhase::PlayerTurn);
        info!("Returning to base hub after extraction");
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

    // Death keeps its accent styling: red border, darker overlay, dark panel.
    spawn_modal_ex(
        &mut commands,
        DeathUiRoot,
        "DEATH",
        520.0,
        380.0,
        60.0,
        false,
        |content| {
            modal_text(
                content,
                "Red has met his end in the Zone",
                FONT_SUB,
                Color::srgb(0.9, 0.9, 0.9),
            );
            modal_text(
                content,
                "Permadeath: stash, inventory and contracts reset to the\nstarter loadout.",
                FONT_BODY,
                COL_TEXT,
            );
        },
        &[hint("E", "New Stalker")],
        Color::srgb(0.8, 0.2, 0.2),
        0.9,
        Color::srgb(0.1, 0.1, 0.1),
    );
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
    mut auto_restart: ResMut<AutoRestartFlag>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        // Mark this transition as a permadeath so prepare_restart_system
        // (OnExit(Running)) performs the full stash/inventory wipe.
        auto_restart.permadeath = true;
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
    /// Set when the run ended via death (permadeath reset required).
    /// Extraction and editor toggles leave Running WITHOUT this flag,
    /// so stash/RunInventory/contracts survive those transitions.
    pub permadeath: bool,
}

/// System that triggers when entering Editing mode from a death/exit
/// Sets a flag to auto-restart the game
///
/// Only performs the permadeath wipe when the transition came from the death
/// screen (marked via AutoRestartFlag::permadeath). Extraction and the F2
/// editor toggle also exit GameState::Running but must preserve the stash,
/// RunInventory (extracted loot) and active contracts.
pub fn prepare_restart_system(
    mut auto_restart: ResMut<AutoRestartFlag>,
    mut contract_system: ResMut<ContractSystem>,
    mut turn_counter: ResMut<TurnCounter>,
    mut message_log: ResMut<MessageLog>,
    mut stash: ResMut<Stash>,
    mut run_inventory: ResMut<RunInventory>,
) {
    // Consume the death marker; without it this exit is extraction or the
    // editor toggle, so leave all run state untouched.
    let permadeath = auto_restart.permadeath;
    auto_restart.permadeath = false;
    if !permadeath {
        info!("Leaving the Zone - stash, RunInventory and contracts preserved");
        return;
    }

    // Reset game state
    contract_system.reset();
    turn_counter.0 = 0;
    message_log.clear();

    // PERMADEATH: Reset stash and run inventory
    stash.clear();
    run_inventory.reset_to_starter();
    info!("PERMADEATH: Reset stash and RunInventory to starter loadout");

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
