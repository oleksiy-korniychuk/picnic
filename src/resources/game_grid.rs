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
    Bolt,
    MetalDetector,
    RustSlag,
}

impl ItemType {
    /// Returns all ItemType variants for dynamic querying
    /// Used by Philosopher's Stone anomaly for dynamic transformations
    pub fn all_variants() -> Vec<ItemType> {
        vec![
            ItemType::FullyEmpty,
            ItemType::Scrap,
            ItemType::GlassJar,
            ItemType::Battery,
            ItemType::Bolt,
            ItemType::MetalDetector,
            ItemType::RustSlag,
        ]
    }
}

#[derive(Resource, Default)]
pub struct SpatialGrid(pub HashMap<Position, Vec<Entity>>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::item::Item;

    #[test]
    fn test_all_item_variants_covered() {
        // This test ensures that all ItemType variants are included in all_variants()
        // If a new ItemType is added but forgotten in all_variants(), this test will fail

        let variants = ItemType::all_variants();

        // Test each variant can be converted to Item and back
        for variant in &variants {
            let item: Item = (*variant).into();
            // If conversion works, the variant is properly defined
            assert!(!item.name.is_empty(), "Item name should not be empty for {:?}", variant);
        }

        // Check count matches expected number of variants
        // Update this number when adding new ItemType variants
        assert_eq!(
            variants.len(),
            7,
            "Expected 7 ItemType variants. If you added a new variant, update this test and all_variants()"
        );

        // Verify no duplicates in all_variants()
        let mut seen = std::collections::HashSet::new();
        for variant in &variants {
            assert!(
                seen.insert(variant),
                "Duplicate variant {:?} in all_variants()",
                variant
            );
        }
    }
}