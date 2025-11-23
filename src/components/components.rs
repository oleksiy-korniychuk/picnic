use bevy::prelude::*;

// --- Core Components ---

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// --- Action Components ---

#[derive(Component, Debug)]
pub struct ActionTravelTo {
    pub destination: Position,
}

#[derive(Component, Debug)]
pub struct ActivePath {
    pub nodes: Vec<Position>,
}

// --- Entity Markers ---

#[derive(Component)]
pub struct EntityMarker;

#[derive(Component)]
pub struct TileMarker;

// --- UI Components ---

#[derive(Component)]
pub struct TickText;

#[derive(Component)]
pub struct PopulationText;

// --- Debug/Visualization ---

#[derive(Component)]
pub struct PathVisualizationEnabled;

#[derive(Component, Debug)]
pub struct PathMarker {
    pub entity: Entity
}

// --- UI: Selection Panel Markers ---

#[derive(Component)]
pub struct SelectedPanelRoot;

#[derive(Component)]
pub struct SelectedEntityIdText;
