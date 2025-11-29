use bevy::prelude::*;
use crate::components::{
    components::Position,
    item::GroundItems,
};
use crate::resources::game_grid::GameGrid;
use crate::systems::rendering::grid_to_world;
use crate::constants::TILE_SIZE;

/// Marker component for ground item sprites
#[derive(Component)]
pub struct GroundItemSprite {
    pub ground_items_entity: Entity,
}

/// Spawns sprites for all GroundItems entities when entering Running mode
pub fn spawn_ground_item_sprites_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ground_items_query: Query<(Entity, &Position, &GroundItems)>,
    existing_sprites: Query<Entity, With<GroundItemSprite>>,
    grid: Res<GameGrid>,
) {
    // Only spawn if sprites don't already exist
    if existing_sprites.iter().next().is_some() {
        return;
    }

    let items_texture = asset_server.load("Items.png");

    for (entity, position, ground_items) in ground_items_query.iter() {
        // Only spawn sprite if there are items on this tile
        if !ground_items.is_empty() {
            let world_pos = grid_to_world(
                position.x as usize,
                position.y as usize,
                grid.width,
                grid.height,
            );

            commands.spawn((
                Sprite {
                    image: items_texture.clone(),
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.6, TILE_SIZE * 0.6)),
                    ..default()
                },
                Transform::from_xyz(world_pos.x, world_pos.y, 1.0), // Z=1, above entities but below player
                GroundItemSprite {
                    ground_items_entity: entity,
                },
            ));
        }
    }
}

/// Despawns all ground item sprites when exiting Running mode
pub fn despawn_ground_item_sprites_system(
    mut commands: Commands,
    sprite_query: Query<Entity, With<GroundItemSprite>>,
) {
    for entity in sprite_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Updates ground item sprites when items are added/removed during Running mode
pub fn update_ground_item_sprites_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ground_items_query: Query<(Entity, &Position, &GroundItems)>,
    sprite_query: Query<(Entity, &GroundItemSprite)>,
    grid: Res<GameGrid>,
) {
    let items_texture = asset_server.load("Items.png");

    // Track which GroundItems entities have sprites
    let mut sprites_map: std::collections::HashMap<Entity, Entity> = sprite_query
        .iter()
        .map(|(sprite_entity, marker)| (marker.ground_items_entity, sprite_entity))
        .collect();

    for (ground_items_entity, position, ground_items) in ground_items_query.iter() {
        if ground_items.is_empty() {
            // Remove sprite if items list is now empty
            if let Some(&sprite_entity) = sprites_map.get(&ground_items_entity) {
                commands.entity(sprite_entity).despawn();
                sprites_map.remove(&ground_items_entity);
            }
        } else {
            // Add sprite if it doesn't exist yet
            if !sprites_map.contains_key(&ground_items_entity) {
                let world_pos = grid_to_world(
                    position.x as usize,
                    position.y as usize,
                    grid.width,
                    grid.height,
                );

                commands.spawn((
                    Sprite {
                        image: items_texture.clone(),
                        custom_size: Some(Vec2::new(TILE_SIZE * 0.6, TILE_SIZE * 0.6)),
                        ..default()
                    },
                    Transform::from_xyz(world_pos.x, world_pos.y, 1.0),
                    GroundItemSprite {
                        ground_items_entity,
                    },
                ));
            }
        }
    }
}
