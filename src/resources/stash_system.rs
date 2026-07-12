use bevy::prelude::*;
use crate::components::item::Item;
use crate::resources::game_grid::ItemType;

/// Persistent storage at base (survives runs until death)
#[derive(Resource, Debug, Default)]
pub struct Stash {
    pub items: Vec<Item>,
    pub capacity: u32,
}

impl Stash {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            capacity: 1000,
        }
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

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

/// Items staged for next run (becomes player inventory when entering zone)
/// Persists at base hub between runs until death
#[derive(Resource, Debug)]
pub struct RunInventory {
    pub items: Vec<Item>,
}

impl Default for RunInventory {
    fn default() -> Self {
        Self::with_starter_loadout()
    }
}

impl RunInventory {
    /// Creates a new RunInventory with starter loadout (10 bolts + metal detector)
    pub fn with_starter_loadout() -> Self {
        let mut items = Vec::new();

        // Add 10 Bolts
        for _ in 0..10 {
            items.push(ItemType::Bolt.into());
        }

        // Add Metal Detector
        items.push(ItemType::MetalDetector.into());

        Self { items }
    }

    pub fn new_empty() -> Self {
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

    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Resets to starter loadout (10 bolts + metal detector)
    pub fn reset_to_starter(&mut self) {
        self.items.clear();

        // Add 10 Bolts
        for _ in 0..10 {
            self.items.push(ItemType::Bolt.into());
        }

        // Add Metal Detector
        self.items.push(ItemType::MetalDetector.into());
    }
}
