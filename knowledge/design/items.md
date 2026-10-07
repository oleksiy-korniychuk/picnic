---
type: Game Design
title: Items and carry weight
description: The POC item catalog with weights and values, carry capacity rules, and the starting loadout.
tags: [picnic, design, items, inventory, in-progress]
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
  - id: analysis
    resource: /references/source-docs/analysis.md
    title: Expert design review and reuse analysis
    author: human:oleksiy
    last_modified: 2025-11-23T22:04:19Z
---

# Item catalog

| Item | Weight | Value | Properties |
|------|--------|-------|------------|
| Bolt | 1 | 1 | Metal; throwable (not yet implemented); starting quantity 10 |
| Fully Empty | 100 | 200 | Artifact |
| Metal Detector | 50 | - | Tool; beeps within 2 tiles; metal |
| Scrap | 10 | 5 | Metal |
| Glass Jar | 5 | 2 | Non-metal |
| Battery | 3 | 3 | Non-metal |
| Rust Slag | 5 | 0 | Byproduct of The Rust; metal |

All items have an `is_metal` field, used by the [metal detector](/systems/inventory.md) and the [Rust anomaly](/design/anomalies.md).[^poc]

# Carry system

* Normal capacity: 250; halved to 125 inside a [Gravitational Anomaly](/design/anomalies.md).[^poc]
* Inventory is "unlimited" (items are always picked up), but the player cannot move while over capacity - movement is blocked with no turn consumed.[^poc]
* Starting loadout: 10 bolts (10 weight) + metal detector (50 weight) = 60 total.[^poc]

# Artifact policy

* Fully Empty is the only artifact in the POC: a basic artifact that seemingly has no use and can simply be sold for money.[^analysis]
* Longer term, artifacts are intended to be both sellable AND usable. The review frames the tension as: "Do I sell this gravity artifact for 5000 rubles, or keep it to manipulate the Whirligig anomaly guarding the Moonlight artifact?" (the Whirligig and Moonlight are illustrative examples, not implemented).[^analysis]

# Money

* Run money is the current stalker's earnings: it starts at 0, is earned by selling items and completing [contracts](/design/contracts.md), and resets on death.[^progression]
* For now it is a pure "net-worth" score - there is no spending yet.[^progression]

[^poc]: POC Design Document
[^progression]: Progression System Design (v0.1)
[^analysis]: Expert design review and reuse analysis
