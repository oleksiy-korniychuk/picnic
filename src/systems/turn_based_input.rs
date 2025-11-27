use bevy::prelude::*;
use crate::components::components::{Player, Position};
use crate::resources::{
    game_grid::{GameGrid, TileKind},
    turn_state::TurnPhase,
};

/// Handles player movement input during PlayerTurn phase
/// WASD moves the player in 4 directions if the destination is valid
pub fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Position, With<Player>>,
    grid: Res<GameGrid>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
) {
    // Get the player's current position
    let Ok(mut player_pos) = player_query.single_mut() else {
        return;
    };

    // Determine movement direction from WASD input
    let mut delta_x = 0;
    let mut delta_y = 0;

    if keyboard.just_pressed(KeyCode::KeyW) {
        delta_y = -1; // Up (negative Y in grid coordinates)
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        delta_y = 1; // Down (positive Y in grid coordinates)
    } else if keyboard.just_pressed(KeyCode::KeyA) {
        delta_x = -1; // Left
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        delta_x = 1; // Right
    }

    // If no movement input, do nothing
    if delta_x == 0 && delta_y == 0 {
        return;
    }

    // Calculate intended destination
    let new_x = player_pos.x + delta_x;
    let new_y = player_pos.y + delta_y;

    // Check if destination is within grid bounds
    if new_x < 0 || new_y < 0 || new_x >= grid.width as i32 || new_y >= grid.height as i32 {
        info!("Movement blocked: out of bounds");
        return;
    }

    // Check if destination tile is walkable (not a wall)
    if let Some(tile) = grid.get_tile(new_x as usize, new_y as usize) {
        if tile.kind == TileKind::Wall {
            info!("Movement blocked: wall at ({}, {})", new_x, new_y);
            return;
        }
    }

    // Valid move - update position and advance to WorldUpdate phase
    player_pos.x = new_x;
    player_pos.y = new_y;

    info!("Player moved to ({}, {})", new_x, new_y);

    // Transition to WorldUpdate phase to process effects
    next_phase.set(TurnPhase::WorldUpdate);
}
