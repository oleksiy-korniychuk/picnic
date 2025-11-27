use bevy::prelude::*;
use crate::components::components::{Player, Position};
use crate::resources::{
    game_grid::{EntityType, GameGrid},
    camera::CameraPosition,
    turn_state::{TurnPhase, TurnCounter},
    message_log::MessageLog,
};
use crate::systems::rendering::grid_to_world;
use crate::constants::TILE_SIZE;

/// Spawns the player entity when entering Running mode
/// Finds PlayerStart marker and spawns player at that location
pub fn spawn_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entity_query: Query<(&Position, &EntityType)>,
    grid: Res<GameGrid>,
    mut camera_position: ResMut<CameraPosition>,
    mut turn_counter: ResMut<TurnCounter>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
    mut message_log: ResMut<MessageLog>,
) {
    // Find the PlayerStart entity
    let player_start_pos = entity_query
        .iter()
        .find_map(|(pos, entity_type)| {
            if matches!(entity_type, EntityType::PlayerStart) {
                Some(*pos)
            } else {
                None
            }
        });

    if let Some(start_pos) = player_start_pos {
        // Load the player sprite
        let texture = asset_server.load("Red.png");

        // Convert grid position to world coordinates
        let world_pos = grid_to_world(
            start_pos.x as usize,
            start_pos.y as usize,
            grid.width,
            grid.height,
        );

        // Spawn player entity
        commands.spawn((
            Sprite {
                image: texture,
                custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                ..default()
            },
            Transform::from_xyz(world_pos.x, world_pos.y, 10.0),
            Player,
            start_pos,
        ));

        // Center camera on player
        camera_position.0 = world_pos;

        // Reset turn counter
        turn_counter.0 = 0;

        // Clear old messages and add spawn message
        message_log.clear();
        message_log.add_message("You enter the Zone...");

        // Start in PlayerTurn phase
        next_phase.set(TurnPhase::PlayerTurn);

        info!("Player spawned at position ({}, {})", start_pos.x, start_pos.y);
    } else {
        warn!("No PlayerStart marker found in the map!");
    }
}

/// Despawns the player entity when exiting Running mode
pub fn despawn_player_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
        info!("Player despawned");
    }
}

/// Syncs the player's visual position (Transform) with their logical Position
/// Runs whenever the player's Position component changes
pub fn sync_player_transform_system(
    mut player_query: Query<(&Position, &mut Transform), With<Player>>,
    grid: Res<GameGrid>,
) {
    for (position, mut transform) in player_query.iter_mut() {
        let world_pos = grid_to_world(
            position.x as usize,
            position.y as usize,
            grid.width,
            grid.height,
        );

        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}
