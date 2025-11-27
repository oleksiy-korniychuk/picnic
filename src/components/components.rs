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

#[derive(Component)]
pub struct Player;

/// Timer for tracking turns inside a Gravitational Anomaly
/// Player dies when this reaches 0
#[derive(Component, Debug)]
pub struct GravitationalAnomalyTimer(pub u32);

// --- UI Components ---

#[derive(Component)]
pub struct TickText;

#[derive(Component)]
pub struct PopulationText;

// --- Game HUD Components ---

#[derive(Component)]
pub struct GameHudRoot;

#[derive(Component)]
pub struct TurnCounterText;

#[derive(Component)]
pub struct WeightText;

#[derive(Component)]
pub struct MessageLogContainer;

#[derive(Component)]
pub struct MessageLogText {
    pub index: usize,  // Which message line (0-4, where 4 is most recent)
}

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
