---
type: Analysis
title: Code reuse analysis
description: How much of the existing real-time Bevy codebase carries over to the turn-based roguelike, and what the POC refactor requires.
tags: [picnic, architecture, refactoring]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: analysis
    resource: /references/source-docs/analysis.md
    title: Expert design review and reuse analysis
    author: human:oleksiy
    last_modified: 2025-11-23T22:04:19Z
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
---

# Directly reusable (~60% of codebase)

* **Camera system** - zoom/pan with boundaries; perfect for a roguelike.[^analysis]
* **Grid-based world** (GameGrid, tile system, coordinates) - ideal for roguelike mechanics.[^analysis]
* **A\* pathfinding** - essential for both player movement and enemy AI.[^analysis]
* **Spatial grid indexing** - critical for fast "what entities are near me?" queries (vision, detection, anomaly effects).[^analysis]
* **Input system framework** - mouse clicks and keyboard already working.[^analysis]
* **Sprite rendering pipeline** - z-layering, visual spawning patterns.[^analysis]
* **UI panel system** - character stats, inventory, minimap.[^analysis]
* **Perlin noise generation** - perfect for procedural Zone generation.[^analysis]

# Easily adaptable (~20%)

* **Intent-Action-Execution pattern** - brilliant for turn-based or real-time roguelike actions. Player intents: WantsToMove, WantsToUseBolt, WantsToThrowItem, WantsToHide. Enemy intents: WantsToPatrol, WantsToInvestigate, WantsToAlert.[^analysis]
* **Movement system** - already 8-directional with cost weighting (perfect for stealth noise mechanics).[^analysis]
* **Parent-child entities** - could be used for equipment visuals, lighting effects.[^analysis]
* **Event system** - detection events, anomaly triggers, artifact interactions.[^analysis]

Bottom line: 60-70% code reuse with smart refactoring; the core engine is rock solid for a grid-based roguelike.[^analysis]

# Refactor requirements for the POC

The existing codebase is real-time ECS. The POC requires:[^poc]

* Turn-based game loop (action queue system).
* Event-driven anomaly processing.
* Inventory/weight component system.
* Text log system.
* Tile-based collision and item management.

# Strengths to lean into

* **Emergent complexity from simple rules.** Anomalies are simple rules that interact unpredictably; artifacts are modifiers to those rules; the environment amplifies them. Examples: electro anomaly + puddle = stun all entities in the puddle; electro anomaly + metal artifact in inventory = you take damage; rubber boots artifact + electro anomaly = immunity.[^analysis]
* **Procedural generation expertise.** Perlin noise is already working: anomaly density maps (higher noise = more anomalies), guard patrol zones (smooth noise = patrol boundaries), artifact rarity (deep noise analysis).[^analysis]
* **Grid-based determinism.** The pathfinding and spatial grid enable bolt physics (throw a bolt, watch it arc, detect anomalies on collision), artifact detection (EMF reader showing a heat map of nearby anomalies), and tactical positioning (hide behind cover, use anomalies to block guards).[^analysis]

[^analysis]: Expert design review and reuse analysis
[^poc]: POC Design Document
