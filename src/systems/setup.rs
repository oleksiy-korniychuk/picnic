use bevy::prelude::*;
use rand::Rng;
use rand_pcg::Pcg32;
use noise::{NoiseFn, Perlin};

use crate::constants::*;
use crate::resources::{
    game_grid::{
        GameGrid,
        TileKind,
        Tile,
        SpatialGrid,
    },
    ui_elements::TickCount,
    seed::WorldSeed,
};

pub fn setup_system(mut commands: Commands) {
    // --- Resource Setup ---
    let world_seed = generate_seed();
    let grid_tiles = generate_height_map(world_seed);

    commands.insert_resource(GameGrid { tiles: grid_tiles });
    commands.insert_resource(SpatialGrid::default());
    commands.insert_resource(TickCount::default());
    commands.insert_resource(WorldSeed(world_seed));
}

pub fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}


// --- Helper Functions ---

fn generate_seed() -> u32 {
    let mut rng = Pcg32::new(
        rand::rng().random_range(0..u64::MAX),
        rand::rng().random_range(0..u64::MAX),
    );
    rng.random_range(0..u32::MAX)
}

fn generate_height_map(seed: u32) -> Vec<Vec<Tile>> {
    let perlin = Perlin::new(seed);
    let mut map = vec![vec![Tile { kind: TileKind::Empty, move_cost: 0 }; GRID_WIDTH]; GRID_HEIGHT];
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let nx = x as f64 * SCALE;
            let ny = y as f64 * SCALE;
            let raw_height = perlin.get([nx, ny]); // Value in [-1, 1]
            let height = ((raw_height + 1.0) / 2.0) as f32; // Normalize to [0,1]
            if height < WATER_LEVEL {
                map[y][x] = Tile { kind: TileKind::Water, move_cost: 100 };
            } else {
                map[y][x] = Tile { kind: TileKind::Dirt, move_cost: 1 };
            }
        }
    }
    map
}

