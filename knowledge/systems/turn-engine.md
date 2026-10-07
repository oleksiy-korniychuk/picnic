---
type: System Spec
title: Turn-based engine
description: The turn state machine (PlayerTurn, WorldUpdate, paused UI phases), processing order, and player mechanics.
resource: src/systems/turn_processor.rs
tags: [picnic, systems, turn-based, live]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
---

# Phases

* Three-phase turn system: `PlayerTurn` (awaiting input), `WorldUpdate` (processing effects), and `InspectingItems` (paused); the full `TurnPhase` state also includes `ViewingInventory` and `ThrowingBolt`.[^poc]
* State-based scheduling uses Bevy's state system; chained world update systems ensure deterministic execution order.[^poc]
* The player spawns/despawns automatically on F2 mode toggle.[^poc]

# Processing order

1. The player inputs movement (WASD), inspection (E), or inventory (Tab) during PlayerTurn (see the [core loop](/design/core-loop.md)).[^poc]
2. Paused phases (optional): the game is paused and no turn advances while the menu is open; ESC closes the UI and advances to WorldUpdate (1 turn consumed).[^poc]
3. WorldUpdate phase (chained systems):[^poc]
   * Gravitational anomaly pull (adjacent tiles).
   * Anomaly effects (see [anomalies](/design/anomalies.md)).
   * Timer updates (gravitational anomaly countdown).
   * Death check (timer reaches 0).
   * Turn counter increment.
   * Transition back to PlayerTurn.

# Player mechanics

* Spawns at the `PlayerStart` marker when entering Running mode; visual representation is the Red.png sprite (80% tile size).[^poc]
* 4-directional WASD movement, only during Running mode in the PlayerTurn phase; movement is blocked by walls or being overweight (no turn consumed if invalid).[^poc]
* Camera auto-follows the player position; panning is disabled during Running mode.[^poc]
* Logical position (`Position` component) is separate from the visual (`Transform`).[^poc]
* Contextual ESC/Tab handling: closes the inspect/inventory UI when open, otherwise exits the game.[^poc]

# Data model

* Resources: `TurnPhase` state (PlayerTurn, WorldUpdate, InspectingItems, ViewingInventory), `TurnCounter`, `CarryCapacity`.[^poc]
* Components: `Player` marker, `GravitationalAnomalyTimer(u32)`, `GroundItems`, `Inventory`.[^poc]
* Game states: `Running` and `Editing` (Paused removed).[^poc]

# Files

`src/systems/player.rs`, `src/systems/turn_based_input.rs`, `src/systems/turn_processor.rs`, `src/resources/turn_state.rs`, `src/systems/inspect_ui.rs`, `src/systems/ground_items.rs`, `src/systems/inventory_ui.rs`, `src/components/inventory.rs`, `src/systems/metal_detector.rs`.[^poc]

[^poc]: POC Design Document
