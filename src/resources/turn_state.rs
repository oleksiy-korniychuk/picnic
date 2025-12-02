use bevy::prelude::*;

/// Represents which phase of the turn we're in
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnPhase {
    #[default]
    PlayerTurn,      // Waiting for player WASD input
    WorldUpdate,     // Processing world effects in sequence
    InspectingItems, // Player is inspecting items on current tile (paused)
    ViewingInventory, // Player is viewing/managing their inventory (paused)
    ThrowingBolt,    // Player is preparing to throw a bolt (paused, waiting for direction)
    EnteringZone,    // Showing contract briefing screen (paused)
    ExitingZone,     // Showing extraction/contract completion screen (paused)
    PlayerDead,      // Showing death screen (paused)
}

/// Tracks the current turn number for display in HUD
#[derive(Resource, Default, Debug)]
pub struct TurnCounter(pub u32);
