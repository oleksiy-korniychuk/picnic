use bevy::prelude::*;
use crate::components::item::Item;

/// Component attached to player representing their carried items
#[derive(Component, Debug, Default, Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn total_weight(&self) -> u32 {
        self.items.iter().map(|item| item.weight).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn has_metal_detector(&self) -> bool {
        self.items.iter().any(|item| item.name == "Metal Detector")
    }
}

/// Resource defining carry capacity limits
#[derive(Resource, Debug)]
pub struct CarryCapacity {
    pub normal: u32,
    pub in_gravity: u32,
}

impl Default for CarryCapacity {
    fn default() -> Self {
        Self {
            normal: 250,
            in_gravity: 125,
        }
    }
}

/// Resource tracking the player's last movement direction for drop mechanics
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LastMoveDirection {
    #[default]
    North,
    South,
    East,
    West,
}

impl LastMoveDirection {
    /// Returns the offset (dx, dy) for this direction
    pub fn offset(&self) -> (i32, i32) {
        match self {
            LastMoveDirection::North => (0, -1),
            LastMoveDirection::South => (0, 1),
            LastMoveDirection::East => (1, 0),
            LastMoveDirection::West => (-1, 0),
        }
    }
}
