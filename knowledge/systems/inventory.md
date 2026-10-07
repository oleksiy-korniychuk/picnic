---
type: System Spec
title: Inventory
description: The inventory UI, weight-based movement restriction, and the metal detector.
resource: src/components/inventory.rs
tags: [picnic, systems, inventory, live]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
---

# Capacity and weight

* Unlimited inventory capacity (items are always picked up), with weight-based movement restriction: movement is blocked if inventory weight exceeds capacity, with no turn consumed.[^poc]
* Capacity varies by context: 250 normal, 125 inside a [Gravitational Anomaly](/design/anomalies.md).[^poc]
* Starting loadout: 10 bolts (10 weight) + metal detector (50 weight) = 60 total.[^poc]

# Inventory UI

* `Tab` opens the modal inventory UI (`ViewingInventory` phase - pauses the game and allows inventory management).[^poc]
* Scrollable list showing all carried items; arrow keys navigate the selection; `D` drops the selected item (see [ground items](/systems/ground-items.md)); `ESC` closes the UI and returns to PlayerTurn (1 turn consumed).[^poc]
* Weight display: "Current/Max" in red if overweight.[^poc]

# Metal detector

* Visual indicator in the top-right corner: "⚠ METAL DETECTED"; no audio (POC).[^poc]
* Only active while the Metal Detector is in the inventory.[^poc]
* Scans a 2-tile radius (Manhattan distance: dx + dy <= 2) for metal ground items via the `is_metal` flag; the indicator shows/hides based on detection.[^poc]

# Data model

* Resources: `CarryCapacity` (normal: 250, in_gravity: 125).[^poc]
* Components: `Inventory` (Vec&lt;Item&gt;), `InventorySelection`, `MetalDetectorIndicator`.[^poc]
* All items have an `is_metal` field for the metal detector and the [Rust anomaly](/design/anomalies.md).[^poc]
* Files: `src/components/inventory.rs`, `src/systems/inventory_ui.rs`, `src/systems/metal_detector.rs`.[^poc]

[^poc]: POC Design Document
