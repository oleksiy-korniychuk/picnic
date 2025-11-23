use bevy::prelude::*;

mod resources;
mod systems;
mod components;
mod constants;

use resources::{
    game_state::GameState,
    camera::{CameraZoom, CameraPosition},
};
use systems::{
    ux::*,
    setup::*,
    input::*,
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
        .add_systems(
            Startup,
            (
                setup_system,
                setup_camera_system,
            ).chain(),
        )
        .add_systems(
            Update,
            (
                toggle_pause_system,
                camera_zoom_system,
                camera_pan_system,
                exit_on_escape_system,
            ),
        )
        .insert_resource(Time::<Fixed>::from_hz(TICK_RATE_HZ))
        .run();
}
