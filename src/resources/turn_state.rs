use bevy::prelude::*;

/// Represents which phase of the turn we're in
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnPhase {
    #[default]
    PlayerTurn,      // Waiting for player WASD input
    WorldUpdate,     // Processing world effects in sequence
    InspectingItems, // Player is inspecting items on current tile (paused)
    ViewingInventory, // Player is viewing/managing their inventory (paused)
}

/// Tracks the current turn number for display in HUD
#[derive(Resource, Default, Debug)]
pub struct TurnCounter(pub u32);
