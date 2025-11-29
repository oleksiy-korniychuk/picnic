use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::resources::game_grid::ItemType;

/// Represents a single item with its properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub weight: u32,
    pub value: Option<u32>, // Some items have no value (tools, bolts, etc.)
    pub is_metal: bool,      // Whether the item is metal (for detector and Rust anomaly)
}

impl Item {
    pub fn new(name: impl Into<String>, weight: u32, value: Option<u32>, is_metal: bool) -> Self {
        Self {
            name: name.into(),
            weight,
            value,
            is_metal,
        }
    }
}

impl From<ItemType> for Item {
    fn from(item_type: ItemType) -> Self {
        match item_type {
            ItemType::FullyEmpty => Item::new("Fully Empty", 100, Some(200), false),
            ItemType::Scrap => Item::new("Scrap", 10, Some(5), true),
            ItemType::GlassJar => Item::new("Glass Jar", 5, Some(2), false),
            ItemType::Battery => Item::new("Battery", 3, Some(3), false),
            ItemType::Bolt => Item::new("Bolt", 1, None, false),
            ItemType::MetalDetector => Item::new("Metal Detector", 50, None, true),
            ItemType::RustSlag => Item::new("Rust Slag", 5, Some(0), true),
        }
    }
}

/// Component attached to tile entities that have items on the ground
/// Multiple items can exist on the same tile
#[derive(Component, Debug, Default, Clone, Serialize, Deserialize)]
pub struct GroundItems {
    pub items: Vec<Item>,
}

impl GroundItems {
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

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}
