---
type: Game Design
title: Anomalies
description: The three POC anomalies - Gravitational Anomaly, Philosopher's Stone, The Rust - their triggers, effects, and bolt-based detection.
tags: [picnic, design, anomalies, live]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
  - id: analysis
    resource: /references/source-docs/analysis.md
    title: Expert design review and reuse analysis
    author: human:oleksiy
    last_modified: 2025-11-23T22:04:19Z
  - id: board-2026-09-30
    resource: /references/idea-board.md
    title: Idea board entry, 2026-09-30
    author: human:oleksiy
---

# Rendering

* **Running mode**: all anomalies appear as identical semi-transparent purple overlays (z-index 2), rendering above the player sprite when the player is on the same tile - so the player cannot identify an anomaly by sight.[^poc]
* **Editor mode**: color-coded for easy placement - purple = Gravitational, gold = Philosopher's Stone, orange = Rust (see the [tilemap editor](/systems/tilemap-editor.md)).[^poc]

# Gravitational Anomaly

* **Pull**: pulls the player in when they are one tile away (4-directional) and the player does not have a timer; the player is pulled 1 tile toward the anomaly during WorldUpdate.[^poc]
* **Effect**: while in the anomaly, carry weight decreases drastically - carry capacity is halved to 125 (see [items](/design/items.md)) - forcing the player to drop items to be able to move out.[^poc]
* **Timer and death**: a timer starts at 5 turns when the player enters the anomaly, decrements each turn the player remains within range (on the anomaly or adjacent), and is only removed when the player escapes to safe distance (> 1 tile away). The player dies (returns to Editing) when the timer reaches 0. Escape requires a minimum of 2 turns: off the anomaly tile, then out of pull range.[^poc]
* **Text**: "You feel as if you weigh a thousand pounds. Every fiber in your body strains and creaks under the weight."[^poc]

# Philosopher's Stone

* **Trigger**: player standing on the anomaly tile with ground items present.[^poc]
* **Effect on valued items**: destroys 1 random item and replaces it with a random different item of a lower or equal tier; there is a small chance (5%) it turns into an [artifact](/design/items.md) (Fully Empty).[^poc]
* **Effect on non-valued items**: shows mysterious flavor text only, no transformation.[^poc]
* **Implementation notes**: dynamic item transformation using `ItemType::all_variants()` for maintainability; atmospheric transformation messages ("The Scrap shimmers and becomes Glass Jar...").[^poc]

# The Rust

* **Trigger**: player standing on the anomaly tile with metal items present (ground or inventory).[^poc]
* **Effect**: at the end of the turn, one metal item on the tile is destroyed and replaced with rust slag.[^poc]
* **Text**: ground items get clear descriptions ("The [item] on the ground begins to rust rapidly..."); inventory items get vague sensory hints ("The acrid smell of oxidation surrounds you...") - the player must infer which item is being destroyed.[^poc]

# Detection design

Bolts are the universal anomaly detector: when a bolt enters an anomaly, text tells the player what they see happen to the bolt, giving clues about what the anomaly might be. Implemented as the [bolt throwing system](/systems/bolt-throwing.md).[^analysis]

A passive perception layer is planned as the [hunch system](/design/hunch.md): reliable radius-based sensory hints near anomalies rather than probability checks, with the radii growing with character experience.[^board-2026-09-30]

Every anomaly needs observable behavior (how the player detects it), interaction mechanics (how the player interacts/avoids it), and artifact integration (how artifacts change the rules).[^analysis]

# Implementation

* Systems: `philosopher_stone_system`, `rust_anomaly_system` in `src/systems/turn_processor.rs`; visual via `update_entity_colors_system` in `src/systems/rendering.rs` (game state-aware).[^poc]
* All anomaly effects generate atmospheric text in the [message log](/systems/hud.md).[^poc]

[^poc]: POC Design Document
[^analysis]: Expert design review and reuse analysis
[^board-2026-09-30]: Idea board entry, 2026-09-30
