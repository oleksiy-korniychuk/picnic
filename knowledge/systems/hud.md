---
type: System Spec
title: HUD
description: Turn counter, weight display, message log, and the metal detector indicator.
resource: src/systems/hud.rs
tags: [picnic, systems, ui, hud, live]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
---

# Display elements

* Turn counter (updates each turn).[^poc]
* Weight display: actual inventory weight / capacity, red text if overweight.[^poc]
* Message log: the last 5 messages, oldest to newest from top to bottom.[^poc]
* Metal detector indicator (when equipped and metal detected).[^poc]
* Positioned at the bottom of the screen with a semi-transparent background; only visible during Running mode (UI components are spawned/despawned with the mode).[^poc]

# Messages

* "You enter the Zone..." (on spawn)[^poc]
* "Gravitational anomaly pulls you in!" (when pulled)[^poc]
* "Immense pressure... 5 turns to escape!" (timer starts)[^poc]
* "Crushing pressure! X turns left!" (each turn in range)[^poc]
* "You break free from the anomaly!" (escaped)[^poc]
* "You are crushed to death!" (death)[^poc]

All [anomaly effects](/design/anomalies.md) generate atmospheric text in the message log.[^poc]

# Implementation

* Resource: `MessageLog` (VecDeque with a max of 5 messages).[^poc]
* Components: `GameHudRoot`, `TurnCounterText`, `WeightText`, `MessageLogText`; change detection for efficient updates.[^poc]
* Styling: semi-transparent black background (rgba 0,0,0,0.8); white monospace text; messages fade slightly with age (newest brightest); stats bar at top, message log below.[^poc]
* Files: `src/resources/message_log.rs`, `src/systems/hud.rs`.[^poc]

[^poc]: POC Design Document
