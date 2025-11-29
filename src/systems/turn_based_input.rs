use bevy::prelude::*;
use crate::components::components::{Player, Position, GravitationalAnomalyTimer};
use crate::components::inventory::{Inventory, CarryCapacity, LastMoveDirection};
use crate::resources::{
    game_grid::{GameGrid, TileKind},
    turn_state::TurnPhase,
    message_log::MessageLog,
};

/// Handles player movement input during PlayerTurn phase
/// WASD moves the player in 4 directions if the destination is valid
pub fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Position, &Inventory, Option<&GravitationalAnomalyTimer>), With<Player>>,
    grid: Res<GameGrid>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
    capacity: Res<CarryCapacity>,
    mut last_direction: ResMut<LastMoveDirection>,
    mut message_log: ResMut<MessageLog>,
) {
    // Get the player's current position and inventory
    let Ok((mut player_pos, inventory, gravity_timer)) = player_query.single_mut() else {
        return;
    };

    // Determine movement direction from WASD input
    let mut delta_x = 0;
    let mut delta_y = 0;
    let mut direction = None;

    if keyboard.just_pressed(KeyCode::KeyW) {
        delta_y = -1; // Up (negative Y in grid coordinates)
        direction = Some(LastMoveDirection::North);
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        delta_y = 1; // Down (positive Y in grid coordinates)
        direction = Some(LastMoveDirection::South);
    } else if keyboard.just_pressed(KeyCode::KeyA) {
        delta_x = -1; // Left
        direction = Some(LastMoveDirection::West);
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        delta_x = 1; // Right
        direction = Some(LastMoveDirection::East);
    }

    // If no movement input, do nothing
    if delta_x == 0 && delta_y == 0 {
        return;
    }

    // Check if player is over carry capacity
    let current_weight = inventory.total_weight();
    let max_capacity = if gravity_timer.is_some() {
        capacity.in_gravity
    } else {
        capacity.normal
    };

    if current_weight > max_capacity {
        message_log.add_message("You're carrying too much weight to move!");
        info!("Movement blocked: over carry capacity ({}/{})", current_weight, max_capacity);
        return;
    }

    // Update last move direction for drop mechanics
    if let Some(dir) = direction {
        *last_direction = dir;
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
