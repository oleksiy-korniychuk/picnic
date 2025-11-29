use bevy::prelude::{Resource, Entity, Component};
use std::collections::HashMap;
use crate::components::components::Position;

#[derive(Resource)]
pub struct GameGrid {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl GameGrid {
    pub fn new_empty(width: usize, height: usize) -> Self {
        let mut tiles = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Tile::new(TileKind::Floor));
            }
            tiles.push(row);
        }

        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y)?.get(x)
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) -> bool {
        if y < self.height && x < self.width {
            self.tiles[y][x] = tile;
            true
        } else {
            false
        }
    }
}

// Terrain layer - what you walk on
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileKind {
    Floor,
    Wall,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub kind: TileKind,
    pub move_cost: i32,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        let move_cost = match kind {
            TileKind::Floor => 1,
            TileKind::Wall => i32::MAX,
        };
        Tile { kind, move_cost }
    }
}

// Entity layer - things placed ON tiles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component)]
pub enum EntityType {
    // Anomalies
    GravitationalAnomaly,
    PhilosopherStone,
    RustAnomaly,
    // Markers
    PlayerStart,
    Exit,
    // Structures
    LampPost,
}

// Item layer - items that can be placed on ground tiles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemType {
    FullyEmpty,
    Scrap,
    GlassJar,
    Battery,
}

#[derive(Resource, Default)]
pub struct SpatialGrid(pub HashMap<Position, Vec<Entity>>);