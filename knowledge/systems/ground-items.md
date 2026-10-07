---
type: System Spec
title: Ground items and inspection
description: Items placed on tiles, the modal inspection UI, and pickup/drop mechanics.
resource: src/systems/ground_items.rs
tags: [picnic, systems, items, live]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
---

# Ground items

* Item data structure with name, weight, and optional value; `GroundItems` component attached to tile positions.[^poc]
* Items are placed on tiles via the editor's Item mode (see the [tilemap editor](/systems/tilemap-editor.md)).[^poc]
* Items are rendered with the `Items.png` sprite (60% tile size, z=1 - above entities, below the player), visible only in Running mode.[^poc]
* Items persist in the map JSON as `items: Vec<PlacedGroundItems>` with backwards-compatible `#[serde(default)]` serialization.[^poc]

# Inspection UI

* `E` while standing on a tile with items opens the modal inspection UI; it pauses gameplay via `TurnPhase::InspectingItems` (the turn does not advance while open).[^poc]
* The modal shows a scrollable list of items with formatted text: "1. ItemName (Weight: X, Value: Y)".[^poc]
* Arrow keys navigate the item list; `E` picks up the selected item into the [inventory](/systems/inventory.md) and removes it from the ground; `ESC` closes the UI (contextually aware - does not exit the game) and consumes 1 turn.[^poc]
* Styling: semi-transparent overlay with a bordered panel (see [UI standards](/systems/ui-standards.md)).[^poc]

# Pickup and drop

* Pickup adds the item to the inventory component, removes it from `GroundItems`, and despawns the ground entity when the last item is picked up; the message log confirms the pickup.[^poc]
* Dropping places the item on the tile in front of the player (based on the last WASD direction), falling back to the player's current tile when blocked; it creates a new `GroundItems` entity or adds to the existing one, and the message log confirms the drop.[^poc]

# Data model

* Components: `Item`, `GroundItems`, `GroundItemSprite`, `InspectUiRoot`.[^poc]
* Files: `src/components/item.rs`, `src/systems/ground_items.rs`, `src/systems/inspect_ui.rs`.[^poc]

[^poc]: POC Design Document
