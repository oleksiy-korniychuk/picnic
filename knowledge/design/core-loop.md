---
type: Game Design
title: Core loop
description: Turn phases, the run structure from base hub to extraction or permadeath, and win/loss conditions.
tags: [picnic, design, core-loop, turn-based, live]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
  - id: progression
    resource: /references/source-docs/v0.1_progression.md
    title: Progression System Design (v0.1)
    author: human:oleksiy
    last_modified: 2025-12-06T05:40:56Z
  - id: board-2025-12-04
    resource: /references/idea-board.md
    title: Idea board entry, 2025-12-04
    author: human:oleksiy
---

# Run structure

```
Base Hub (Stash + Contracts)
  ↓
Prepare Loadout (move items: stash → run inventory)
  ↓
Enter Zone
  ↓
Survive + Loot
  ↓
Extract OR Die
  ↓
├─ Extract → Return to Base with loot
│   ├─ Move items between run inventory ↔ stash
│   ├─ Sell items for money
│   ├─ Turn in items for contracts (bonus rewards)
│   └─ Repeat
│
└─ Die → PERMADEATH
    ├─ New stalker, reset stash + contracts + money
    ├─ Map knowledge persists (explored terrain kept)
    └─ Start over
```

[^progression]

Each stalker has one life - death resets stash, contracts, and money while map knowledge persists;[^board-2025-12-04] see [progression](/design/progression.md) for permadeath rules, the base hub, and item flow.[^progression]

# Turn phases

The turn system has three phases: `PlayerTurn` (awaiting input), `WorldUpdate` (processing effects), and paused inspection phases. See the [turn-based engine](/systems/turn-engine.md) for the full state machine and processing order.[^poc]

1. The player inputs movement (WASD), inspection (E), or inventory (Tab) during PlayerTurn:[^poc]
   * Movement validates weight, updates position, and advances to WorldUpdate (1 turn).
   * Inspection opens the [ground items](/systems/ground-items.md) modal and pauses the game; no turn consumed yet.
   * Inventory opens the [inventory](/systems/inventory.md) modal and pauses the game; no turn consumed yet.
2. Paused phases (`InspectingItems`, `ViewingInventory`, `ThrowingBolt`): the game is paused and no turn advances while the menu is open; ESC closes the UI and advances to WorldUpdate (1 turn consumed).[^poc]
3. WorldUpdate runs chained systems in deterministic order: gravitational anomaly pull, anomaly effects, timer updates, death check, turn counter increment, then back to PlayerTurn.[^poc]

Movement is blocked by walls or being overweight, with no turn consumed when the move is invalid.[^poc]

# Win condition

* A mission briefing screen on zone entry (EnteringZone phase) shows the active [contracts](/design/contracts.md); press E to accept and begin.[^poc]
* Reaching the exit tile shows the extraction screen (ExitingZone phase) with contract completion status marked `[COMPLETE]`/`[FAILED]`; press E to exit the zone, return to the [base hub](/design/progression.md), and restart with new contracts.[^poc]

# Loss condition

* Death in a [Gravitational Anomaly](/design/anomalies.md) (timer reaches 0) shows the death screen - "Red has met his end in the Zone"; press E to restart with a new stalker.[^poc]

# Auto-restart

* Death and exit transition through Editing and auto-restart to Running; all game state resets (contracts, turn counter, message log).[^poc]

[^poc]: POC Design Document
[^progression]: Progression System Design (v0.1)
[^board-2025-12-04]: Idea board entry, 2025-12-04
