use bevy::prelude::*;
use crate::resources::game_grid::{TileKind, EntityType, ItemType};

#[derive(Resource)]
pub struct EditorState {
    pub mode: EditorMode,
    pub selected_terrain: TileKind,
    pub selected_entity: EntityType,
    pub selected_item: ItemType,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            mode: EditorMode::Terrain,
            selected_terrain: TileKind::Floor,
            selected_entity: EntityType::GravitationalAnomaly,
            selected_item: ItemType::FullyEmpty,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Terrain,
    Entity,
    Item,
}

#[derive(Resource, Default)]
pub struct EditorCursor {
    pub grid_position: Option<(usize, usize)>,
}
