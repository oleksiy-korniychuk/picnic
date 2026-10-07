use bevy::prelude::*;
use crate::components::item::Item;
use crate::resources::game_grid::ItemType;

/// Persistent storage at base (survives runs until death)
#[derive(Resource, Debug)]
pub struct Stash {
    pub items: Vec<Item>,
    pub capacity: u32,
}

impl Default for Stash {
    /// init_resource uses this, so the default must carry the designed
    /// 1000 capacity (previously a derived Default gave capacity 0).
    fn default() -> Self {
        Self {
            items: Vec::new(),
            capacity: 1000,
        }
    }
}

impl Stash {
    pub fn new() -> Self {
        Self::default()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_stash_has_designed_capacity() {
        // init_resource::<Stash>() uses Default; a derived Default would give
        // capacity 0 and silently break any future capacity enforcement.
        assert_eq!(Stash::default().capacity, 1000);
        assert!(Stash::default().is_empty());
    }

    #[test]
    fn run_inventory_default_is_starter_loadout() {
        let inv = RunInventory::default();
        assert_eq!(inv.count(), 11);
        assert_eq!(inv.total_weight(), 60);
    }
}
