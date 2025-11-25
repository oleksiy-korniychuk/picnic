use bevy::prelude::*;

mod resources;
mod systems;
mod components;
mod constants;

use resources::{
    game_state::GameState,
    camera::{CameraZoom, CameraPosition},
    editor_state::{EditorState, EditorCursor},
};
use systems::{
    ux::*,
    setup::*,
    input::*,
    editor::*,
    rendering::*,
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
        .init_resource::<CameraZoom>()
        .init_resource::<CameraPosition>()
        .init_resource::<EditorState>()
        .init_resource::<EditorCursor>()
        .add_systems(
            Startup,
            (
                setup_system,
                setup_camera_system,
                spawn_tile_sprites_system,
                spawn_editor_hud_system,
            ).chain(),
        )
        .add_systems(
            Update,
            (
                // Always active
                editor_toggle_system,
                toggle_pause_system,
                camera_zoom_system,
                camera_pan_system,
                exit_on_escape_system,
                toggle_editor_hud_visibility_system,
                update_tile_sprite_system,
                reload_tile_sprites_system,
            ),
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
            ).run_if(in_state(GameState::Editing)),
        )
        .insert_resource(Time::<Fixed>::from_hz(TICK_RATE_HZ))
        .run();
}
