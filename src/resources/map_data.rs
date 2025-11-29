use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::resources::game_grid::{GameGrid, TileKind, EntityType, Tile};
use crate::components::item::{Item, GroundItems};

#[derive(Serialize, Deserialize, Debug)]
pub struct MapData {
    pub width: usize,
    pub height: usize,
    pub terrain: Vec<Vec<SerializableTileKind>>,
    pub entities: Vec<PlacedEntity>,
    #[serde(default)] // Backwards compatible - defaults to empty vec if missing
    pub items: Vec<PlacedGroundItems>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlacedGroundItems {
    pub x: usize,
    pub y: usize,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SerializableTileKind {
    Floor,
    Wall,
}

impl From<TileKind> for SerializableTileKind {
    fn from(kind: TileKind) -> Self {
        match kind {
            TileKind::Floor => SerializableTileKind::Floor,
            TileKind::Wall => SerializableTileKind::Wall,
        }
    }
}

impl From<SerializableTileKind> for TileKind {
    fn from(kind: SerializableTileKind) -> Self {
        match kind {
            SerializableTileKind::Floor => TileKind::Floor,
            SerializableTileKind::Wall => TileKind::Wall,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SerializableEntityType {
    GravitationalAnomaly,
    PhilosopherStone,
    RustAnomaly,
    PlayerStart,
    Exit,
    LampPost,
}

impl From<EntityType> for SerializableEntityType {
    fn from(et: EntityType) -> Self {
        match et {
            EntityType::GravitationalAnomaly => SerializableEntityType::GravitationalAnomaly,
            EntityType::PhilosopherStone => SerializableEntityType::PhilosopherStone,
            EntityType::RustAnomaly => SerializableEntityType::RustAnomaly,
            EntityType::PlayerStart => SerializableEntityType::PlayerStart,
            EntityType::Exit => SerializableEntityType::Exit,
            EntityType::LampPost => SerializableEntityType::LampPost,
        }
    }
}

impl From<SerializableEntityType> for EntityType {
    fn from(et: SerializableEntityType) -> Self {
        match et {
            SerializableEntityType::GravitationalAnomaly => EntityType::GravitationalAnomaly,
            SerializableEntityType::PhilosopherStone => EntityType::PhilosopherStone,
            SerializableEntityType::RustAnomaly => EntityType::RustAnomaly,
            SerializableEntityType::PlayerStart => EntityType::PlayerStart,
            SerializableEntityType::Exit => EntityType::Exit,
            SerializableEntityType::LampPost => EntityType::LampPost,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlacedEntity {
    pub entity_type: SerializableEntityType,
    pub x: usize,
    pub y: usize,
}

impl MapData {
    // Create MapData from current game state
    pub fn from_game_state(
        grid: &GameGrid,
        entities: &[(EntityType, usize, usize)],
        ground_items: &[(GroundItems, usize, usize)],
    ) -> Self {
        let mut terrain = Vec::with_capacity(grid.height);
        for y in 0..grid.height {
            let mut row = Vec::with_capacity(grid.width);
            for x in 0..grid.width {
                if let Some(tile) = grid.get_tile(x, y) {
                    row.push(tile.kind.into());
                } else {
                    row.push(SerializableTileKind::Floor);
                }
            }
            terrain.push(row);
        }

        let entities = entities
            .iter()
            .map(|(entity_type, x, y)| PlacedEntity {
                entity_type: (*entity_type).into(),
                x: *x,
                y: *y,
            })
            .collect();

        let items = ground_items
            .iter()
            .map(|(ground_items, x, y)| PlacedGroundItems {
                x: *x,
                y: *y,
                items: ground_items.items.clone(),
            })
            .collect();

        MapData {
            width: grid.width,
            height: grid.height,
            terrain,
            entities,
            items,
        }
    }

    // Save to JSON file
    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        // Ensure directory exists
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        fs::write(path, json)
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(())
    }

    // Load from JSON file
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let map_data: MapData = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to deserialize: {}", e))?;

        Ok(map_data)
    }

    // Convert MapData to GameGrid
    pub fn to_game_grid(&self) -> GameGrid {
        let mut tiles = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut row = Vec::with_capacity(self.width);
            for x in 0..self.width {
                let tile_kind = if y < self.terrain.len() && x < self.terrain[y].len() {
                    self.terrain[y][x].into()
                } else {
                    TileKind::Floor
                };
                row.push(Tile::new(tile_kind));
            }
            tiles.push(row);
        }

        GameGrid {
            tiles,
            width: self.width,
            height: self.height,
        }
    }
}
