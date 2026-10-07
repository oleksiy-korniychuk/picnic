---
type: System Spec
title: Tilemap editor
description: In-game editor for dynamic-sized tile maps with terrain, entity, and item placement modes, and JSON save/load.
resource: src/systems/editor.rs
tags: [picnic, systems, editor, live]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
---

# Access and modes

* `F2` toggles between Running and Editing modes (also handles the InBaseHub state).[^poc]
* `Tab` cycles placement modes: Terrain → Entity → Item.[^poc]
* Mode-dependent key bindings: each mode starts at key 1 (no shared number row).[^poc]

## Terrain mode

* `1`: Floor, `2`: Wall.[^poc]

## Entity mode

* `1`: Gravitational Anomaly, `2`: Philosopher's Stone, `3`: Rust Anomaly, `4`: Player Start, `5`: Exit, `6`: Lamp Post.[^poc]

## Item mode

* `1`: Fully Empty (artifact, 100 weight, 200 value), `2`: Scrap (10 weight, 5 value), `3`: Glass Jar (5, 2), `4`: Battery (3, 3), `5`: Bolt, `6`: Metal Detector, `7`: Rust Slag.[^poc]

# Placement controls

* Left click places the selected terrain/entity/item; multiple items can stack on the same tile.[^poc]
* Right click deletes an entity, resets the tile to Floor, or removes all items from the tile.[^poc]
* `F3` quick-saves to `assets/maps/current.json`; `F4` quick-loads from it.[^poc]

# Architecture

* Separated terrain layer (Floor/Wall) from the entity layer (anomalies/items/markers).[^poc]
* Dynamic grid sizing - defaults to 25x25, supports any size on load.[^poc]
* ECS-based: tiles and entities are proper Bevy entities with `Position` components.[^poc]
* JSON serialization via serde for map save/load (backwards-compatible `items` field); automatic tile/entity/item sprite reload on map load.[^poc]
* Keyboard-only interface (no complex UI forms); grid coordinates properly convert to/from world space.[^poc]

# Visual feedback

* Gray tiles for Floor, dark gray for Walls.[^poc]
* Color-coded entities: purple/gold/orange for anomalies, green for player start, blue for exit.[^poc]
* `Items.png` sprite on tiles with items (only visible in Item mode in the editor).[^poc]
* White semi-transparent cursor highlight showing the current grid position.[^poc]
* Minimal HUD displaying mode, mode-specific current selection, and cursor coordinates.[^poc]

# Files

`src/systems/editor.rs`, `src/systems/rendering.rs`, `src/resources/map_data.rs`, `src/components/item.rs`.[^poc]

[^poc]: POC Design Document
