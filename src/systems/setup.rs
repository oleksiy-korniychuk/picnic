use bevy::prelude::*;
use rand::Rng;
use rand_pcg::Pcg32;

use crate::resources::{
    game_grid::{
        GameGrid,
        SpatialGrid,
    },
    ui_elements::TickCount,
    seed::WorldSeed,
};

pub fn setup_system(mut commands: Commands) {
    // --- Resource Setup ---
    let world_seed = generate_seed();

    // Default to 25x25 grid - can be changed by loading different sized maps
    let game_grid = GameGrid::new_empty(25, 25);

    commands.insert_resource(game_grid);
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

