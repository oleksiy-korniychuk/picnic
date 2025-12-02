use bevy::prelude::*;

mod resources;
mod systems;
mod components;
mod constants;

use resources::{
    game_state::GameState,
    camera::{CameraZoom, CameraPosition},
    editor_state::{EditorState, EditorCursor},
    turn_state::{TurnPhase, TurnCounter},
    message_log::MessageLog,
    contract_system::ContractSystem,
};
use components::inventory::CarryCapacity;
use systems::{
    setup::*,
    input::*,
    editor::*,
    rendering::*,
    player::*,
    turn_based_input::*,
    turn_processor::*,
    hud::*,
    ground_items::*,
    inspect_ui::*,
    inventory_ui::*,
    metal_detector::*,
    contract_ui::*,
};
use constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Picnic".into(),
                resolution: (
                    DEFAULT_WINDOW_WIDTH,
                    DEFAULT_WINDOW_HEIGHT,
                ).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_state::<TurnPhase>()
        .init_resource::<CameraZoom>()
        .init_resource::<CameraPosition>()
        .init_resource::<EditorState>()
        .init_resource::<EditorCursor>()
        .init_resource::<TurnCounter>()
        .init_resource::<MessageLog>()
        .init_resource::<CarryCapacity>()
        .init_resource::<ContractSystem>()
        .init_resource::<AutoRestartFlag>()
        .add_systems(
            Startup,
            (
                setup_system,
                setup_camera_system,
                spawn_tile_sprites_system,
                spawn_editor_hud_system,
            ).chain(),
        )
        .add_systems(OnEnter(GameState::Running), (
            spawn_player_system,
            set_entering_zone_phase_system,
            spawn_game_hud_system,
            spawn_ground_item_sprites_system,
            spawn_metal_detector_indicator_system,
        ).chain())
        .add_systems(OnExit(GameState::Running), (
            despawn_player_system,
            despawn_game_hud_system,
            despawn_ground_item_sprites_system,
            despawn_metal_detector_indicator_system,
            prepare_restart_system,
        ))
        .add_systems(
            Update,
            (
                // Always active
                editor_toggle_system,
                camera_zoom_system,
                exit_on_escape_system,
                toggle_editor_hud_visibility_system,
                update_tile_sprite_system,
                reload_tile_sprites_system,
                update_entity_colors_system,
            ),
        )
        .add_systems(
            Update,
            (
                // Camera pan only in Editing mode
                camera_pan_system,
            ).run_if(in_state(GameState::Editing)),
        )
        .add_systems(
            Update,
            (
                // Running mode - camera follows player, player transform syncs, HUD updates, ground items, metal detector
                camera_follow_player_system,
                sync_player_transform_system,
                update_turn_counter_system,
                update_weight_display_system,
                update_message_log_system,
                update_ground_item_sprites_system,
                update_metal_detector_system,
            ).run_if(in_state(GameState::Running)),
        )
        .add_systems(
            Update,
            (
                // PlayerTurn phase - handle movement input, item inspection, inventory, and exit detection
                player_movement_system,
                detect_inspect_input_system,
                detect_inventory_input_system,
                detect_exit_system,
            ).run_if(in_state(GameState::Running))
             .run_if(in_state(TurnPhase::PlayerTurn)),
        )
        .add_systems(
            Update,
            (
                // WorldUpdate phase - chained systems in exact order
                gravitational_pull_system,
                philosopher_stone_system,
                rust_anomaly_system,
                gravitational_timer_system,
                death_check_system,
                increment_turn_counter_system,
                transition_to_player_turn_system,
            ).chain()
             .run_if(in_state(GameState::Running))
             .run_if(in_state(TurnPhase::WorldUpdate)),
        )
        .add_systems(OnEnter(TurnPhase::InspectingItems), (
            spawn_inspect_ui_system,
        ))
        .add_systems(OnExit(TurnPhase::InspectingItems), (
            despawn_inspect_ui_system,
        ))
        .add_systems(
            Update,
            (
                // InspectingItems phase - handle navigation, pickup, and ESC to close
                close_inspect_ui_system,
                inspect_navigation_system,
                pickup_item_system,
                update_inspect_ui_selection_system,
                rebuild_inspect_ui_system,
            ).run_if(in_state(GameState::Running))
             .run_if(in_state(TurnPhase::InspectingItems)),
        )
        .add_systems(OnEnter(TurnPhase::ViewingInventory), (
            spawn_inventory_ui_system,
        ))
        .add_systems(OnExit(TurnPhase::ViewingInventory), (
            despawn_inventory_ui_system,
        ))
        .add_systems(
            Update,
            (
                // ViewingInventory phase - handle inventory interactions
                close_inventory_ui_system,
                inventory_navigation_system,
                drop_item_system,
                update_inventory_ui_selection_system,
                rebuild_inventory_ui_system,
                auto_scroll_inventory_system,
            ).run_if(in_state(GameState::Running))
             .run_if(in_state(TurnPhase::ViewingInventory)),
        )
        .add_systems(OnEnter(TurnPhase::EnteringZone), (
            spawn_enter_zone_ui_system,
        ))
        .add_systems(OnExit(TurnPhase::EnteringZone), (
            despawn_enter_zone_ui_system,
        ))
        .add_systems(
            Update,
            (
                // EnteringZone phase - show contract briefing
                close_enter_zone_ui_system,
            ).run_if(in_state(GameState::Running))
             .run_if(in_state(TurnPhase::EnteringZone)),
        )
        .add_systems(OnEnter(TurnPhase::ExitingZone), (
            spawn_exit_zone_ui_system,
        ))
        .add_systems(OnExit(TurnPhase::ExitingZone), (
            despawn_exit_zone_ui_system,
        ))
        .add_systems(
            Update,
            (
                // ExitingZone phase - show contract completion status
                close_exit_zone_ui_system,
            ).run_if(in_state(GameState::Running))
             .run_if(in_state(TurnPhase::ExitingZone)),
        )
        .add_systems(OnEnter(TurnPhase::PlayerDead), (
            spawn_death_ui_system,
        ))
        .add_systems(OnExit(TurnPhase::PlayerDead), (
            despawn_death_ui_system,
        ))
        .add_systems(
            Update,
            (
                // PlayerDead phase - show death screen
                close_death_ui_system,
            ).run_if(in_state(GameState::Running))
             .run_if(in_state(TurnPhase::PlayerDead)),
        )
        .add_systems(
            Update,
            (
                // Editor-only systems
                editor_mode_toggle_system,
                editor_selection_system,
                editor_mouse_position_system,
                editor_cursor_highlight_system,
                editor_placement_system,
                editor_save_load_system,
                update_editor_hud_system,
                auto_restart_system,
            ).run_if(in_state(GameState::Editing)),
        )
        .insert_resource(Time::<Fixed>::from_hz(TICK_RATE_HZ))
        .run();
}
