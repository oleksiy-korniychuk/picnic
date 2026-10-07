---
type: System Spec
title: Bolt throwing
description: Throwing bolts to detect anomalies - controls, ballistics, animation, and collision feedback.
resource: src/systems/bolt_throwing.rs
tags: [picnic, systems, anomalies, detection, live]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
---

# Controls

* `Q` enters the `ThrowingBolt` phase (requires a bolt in the [inventory](/systems/inventory.md)).[^poc]
* `WASD` selects a direction; the bolt fires automatically after the direction is selected; `ESC` cancels and returns to PlayerTurn.[^poc]
* A red square indicator appears during direction selection.[^poc]

# Mechanics

* Range: 5 tiles in a straight line (4 directions); consumes 1 bolt from the inventory.[^poc]
* Collision: stops at walls, [anomalies](/design/anomalies.md), or max range.[^poc]
* Fully integrated with the inventory system.[^poc]

# Detection feedback

* Reports the anomaly type when the bolt collides with one - this is the primary anomaly-detection mechanic: the text tells the player what they see happen to the bolt, giving clues about what the anomaly might be.[^poc]
* Reports wall/obstacle collisions and empty tiles at max range via the message log ([HUD](/systems/hud.md)).[^poc]

# Visuals

* Animation: 0.5 second flight, tile-by-tile, with a fading trail; the bolt sprite animates along the path; trail sprites fade out after the bolt passes; all sprites are cleaned up automatically.[^poc]

# Implementation

* Components: `BoltThrowingIndicator`, `BoltProjectile`, `BoltTrail`.[^poc]
* Systems: `detect_bolt_throw_input_system`, `spawn_bolt_indicator_system`, `bolt_direction_input_system`, `animate_bolt_flight_system`, `update_bolt_trail_system`, `despawn_bolt_indicator_system`.[^poc]
* Files: `src/systems/bolt_throwing.rs`.[^poc]

[^poc]: POC Design Document
