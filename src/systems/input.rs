use bevy::prelude::*;
use bevy::input::mouse::{
    MouseScrollUnit,
    MouseWheel,
};
use bevy::app::AppExit;

use crate::constants::*;
use crate::resources::{
    camera::{CameraZoom, CameraPosition},
    game_grid::GameGrid,
};

pub fn exit_on_escape_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

pub fn camera_zoom_system(
    mut commands: Commands,
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_zoom: ResMut<CameraZoom>,
    camera_query: Query<Entity, With<Camera2d>>,
    windows: Query<&Window>,
    grid: Res<GameGrid>,
) {
    for ev in scroll_evr.read() {
        let zoom_delta = match ev.unit {
            MouseScrollUnit::Line => ev.y * ZOOM_SPEED * camera_zoom.0,
            MouseScrollUnit::Pixel => ev.y * ZOOM_SPEED * 0.01 * camera_zoom.0,
        };

        let max_zoom = if let Ok(window) = windows.single() {
            let map_width = grid.width as f32 * TILE_SIZE;
            let map_height = grid.height as f32 * TILE_SIZE;

            let scale_for_width = map_width / window.width();
            let scale_for_height = map_height / window.height();
            let max_zoom_out = scale_for_width.max(scale_for_height);

            max_zoom_out
        } else {
            5.0
        };

        // Update zoom level
        camera_zoom.0 = (camera_zoom.0 - zoom_delta).clamp(MIN_ZOOM, max_zoom);

        // Apply zoom to camera
        if let Ok(camera_entity) = camera_query.single() {
            commands.entity(camera_entity).insert(Projection::from(OrthographicProjection {
                scale: camera_zoom.0,
                ..OrthographicProjection::default_2d()
            }));
        }
    }
}

pub fn camera_pan_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_position: ResMut<CameraPosition>,
    camera_query: Query<Entity, With<Camera2d>>,
    time: Res<Time>,
    camera_zoom: Res<CameraZoom>,
    windows: Query<&Window>,
    grid: Res<GameGrid>,
) {
    let mut pan_direction = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        pan_direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        pan_direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        pan_direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        pan_direction.x += 1.0;
    }

    if pan_direction != Vec2::ZERO {
        // Normalize the direction to prevent faster diagonal movement
        pan_direction = pan_direction.normalize();

        // Scale pan speed by zoom level so panning feels consistent
        let pan_speed = CAMERA_PAN_SPEED * camera_zoom.0 * time.delta_secs();
        let new_position = camera_position.0 + pan_direction * pan_speed;

        // Calculate map boundaries using dynamic grid size
        let map_half_width = grid.width as f32 * TILE_SIZE / 2.0;
        let map_half_height = grid.height as f32 * TILE_SIZE / 2.0;

        // Calculate viewport size based on zoom and window size
        if let Ok(window) = windows.single() {
            let viewport_half_width = window.width() * camera_zoom.0 / 2.0;
            let viewport_half_height = window.height() * camera_zoom.0 / 2.0;

            // If viewport is larger than map, center camera and don't allow panning
            if viewport_half_width >= map_half_width || viewport_half_height >= map_half_height {
                camera_position.0 = Vec2::ZERO;
            } else {
                // Calculate bounds that keep the viewport within the map
                let min_x = -map_half_width + viewport_half_width;
                let max_x = map_half_width - viewport_half_width;
                let min_y = -map_half_height + viewport_half_height;
                let max_y = map_half_height - viewport_half_height;

                // Apply boundary constraints
                camera_position.0.x = new_position.x.clamp(min_x, max_x);
                camera_position.0.y = new_position.y.clamp(min_y, max_y);
            }
        } else {
            // Fallback - just apply the movement without bounds
            camera_position.0 = new_position;
        };

        // Apply the new position to the camera
        if let Ok(camera_entity) = camera_query.single() {
            commands.entity(camera_entity).insert(Transform::from_translation(
                camera_position.0.extend(0.0)
            ));
        }
    }
}


