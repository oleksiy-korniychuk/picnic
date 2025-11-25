use bevy::prelude::*;
use crate::resources::game_grid::{GameGrid, TileKind, EntityType};
use crate::components::components::{Position, TileMarker};
use crate::constants::TILE_SIZE;

// Component to link tile entities to their grid position
#[derive(Component)]
pub struct TileEntity {
    pub grid_x: usize,
    pub grid_y: usize,
}

// Spawn all tile sprites at startup
pub fn spawn_tile_sprites_system(
    mut commands: Commands,
    grid: Res<GameGrid>,
) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let tile = &grid.tiles[y][x];
            let color = get_tile_color(tile.kind);

            // Calculate world position
            // Grid coordinates: (0,0) is top-left
            // World coordinates: (0,0) is center
            let world_x = (x as f32 - grid.width as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0;
            let world_y = (grid.height as f32 / 2.0 - y as f32) * TILE_SIZE - TILE_SIZE / 2.0;

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, -10.0),
                TileMarker,
                TileEntity { grid_x: x, grid_y: y },
                Position {
                    x: x as i32,
                    y: y as i32
                },
            ));
        }
    }
}

// Get color for terrain tiles
fn get_tile_color(kind: TileKind) -> Color {
    match kind {
        TileKind::Floor => Color::srgb(0.33, 0.33, 0.33), // Gray
        TileKind::Wall => Color::srgb(0.13, 0.13, 0.13),  // Dark gray
    }
}

// Get color for entity types
pub fn get_entity_color(entity_type: EntityType) -> Color {
    match entity_type {
        EntityType::GravitationalAnomaly => Color::srgb(0.53, 0.0, 1.0), // Purple
        EntityType::PhilosopherStone => Color::srgb(1.0, 0.84, 0.0),     // Gold
        EntityType::RustAnomaly => Color::srgb(1.0, 0.4, 0.0),           // Orange
        EntityType::PlayerStart => Color::srgb(0.0, 1.0, 0.0),           // Green
        EntityType::Exit => Color::srgb(0.0, 0.53, 1.0),                 // Blue
        EntityType::LampPost => Color::srgb(1.0, 1.0, 0.0),              // Yellow
        EntityType::FullyEmpty => Color::srgb(0.8, 0.5, 0.9),            // Light purple
    }
}

// Update tile sprites when terrain changes (called from editor)
pub fn update_tile_sprite_system(
    grid: Res<GameGrid>,
    mut tile_query: Query<(&TileEntity, &mut Sprite), With<TileMarker>>,
) {
    if !grid.is_changed() {
        return;
    }

    for (tile_entity, mut sprite) in tile_query.iter_mut() {
        if let Some(tile) = grid.get_tile(tile_entity.grid_x, tile_entity.grid_y) {
            sprite.color = get_tile_color(tile.kind);
        }
    }
}

// Helper to convert grid coords to world position
pub fn grid_to_world(grid_x: usize, grid_y: usize, grid_width: usize, grid_height: usize) -> Vec2 {
    let world_x = (grid_x as f32 - grid_width as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0;
    let world_y = (grid_height as f32 / 2.0 - grid_y as f32) * TILE_SIZE - TILE_SIZE / 2.0;
    Vec2::new(world_x, world_y)
}

// Spawn an entity at a grid position
pub fn spawn_placed_entity(
    commands: &mut Commands,
    entity_type: EntityType,
    grid_x: usize,
    grid_y: usize,
    grid_width: usize,
    grid_height: usize,
) {
    let world_pos = grid_to_world(grid_x, grid_y, grid_width, grid_height);
    let color = get_entity_color(entity_type);

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)), // Slightly smaller than tiles
            ..default()
        },
        Transform::from_xyz(world_pos.x, world_pos.y, 0.0), // Z=0 for entities, above tiles
        entity_type,
        Position {
            x: grid_x as i32,
            y: grid_y as i32,
        },
    ));
}

// Reload all tile sprites when grid dimensions change (e.g., after loading a map)
// This system detects when the grid resource changes and respawns all tiles
pub fn reload_tile_sprites_system(
    mut commands: Commands,
    grid: Res<GameGrid>,
    tile_query: Query<Entity, With<TileMarker>>,
) {
    // Only run when grid changes (indicates a map load)
    if !grid.is_changed() {
        return;
    }

    // Check if grid dimensions changed by counting existing tiles
    let existing_tile_count = tile_query.iter().count();
    let expected_tile_count = grid.width * grid.height;

    if existing_tile_count != expected_tile_count {
        // Despawn all existing tiles
        for entity in tile_query.iter() {
            commands.entity(entity).despawn();
        }

        // Respawn all tiles with new grid dimensions
        for y in 0..grid.height {
            for x in 0..grid.width {
                let tile = &grid.tiles[y][x];
                let color = get_tile_color(tile.kind);

                let world_x = (x as f32 - grid.width as f32 / 2.0) * TILE_SIZE + TILE_SIZE / 2.0;
                let world_y = (grid.height as f32 / 2.0 - y as f32) * TILE_SIZE - TILE_SIZE / 2.0;

                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(world_x, world_y, -10.0),
                    TileMarker,
                    TileEntity { grid_x: x, grid_y: y },
                    Position {
                        x: x as i32,
                        y: y as i32
                    },
                ));
            }
        }
    }
}
