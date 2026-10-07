---
type: Design Decision
title: Design decisions from the expert review
description: Resolved answers to the design critique - core loop style, moment-to-moment play, win condition, anomaly detection, artifacts, day/night, and economy.
tags: [picnic, design, decisions]
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
  - id: board-2025-12-04
    resource: /references/idea-board.md
    title: Idea board entry, 2025-12-04
    author: human:oleksiy
---

# Core loop style

* Turn-based, handcrafted open-world Zone.[^analysis]
* Progression is character based for skills and items, exploration based, and meta-game knowledge based.[^analysis]

# Moment-to-moment play

* The player slowly moves through the Zone, expands the known map, observes and avoids anomalies, and collects artifacts and mundane items.[^analysis]
* Rough split: ~70% movement/exploration, ~30% managing artifacts/items; no combat. This 70/30 split is the starting point.[^analysis]

# Win condition and scope

* Mission-based runs: each run starts with the player selecting one of several available contracts which pay extra for being completed, on top of the sale value of collected items (see [contracts](/design/contracts.md)).[^analysis]
* Exploration-based: uncover the map, reach new areas of the Zone, escape.[^analysis]
* Not survival-based (no last-as-long-as-possible high score).[^analysis]

# Anomaly detection

* All anomalies are detected by throwing bolts: when a bolt enters an anomaly, text tells the player what they see happen to the bolt, giving clues about what the anomaly might be. Implemented as the [bolt throwing system](/systems/bolt-throwing.md).[^analysis]
* Anomalies need observable behavior (how the player detects them), interaction mechanics (how to interact/avoid), and artifact integration (how artifacts change the rules).[^analysis]

# Artifacts in the POC

* Only Fully Empty artifacts spawn for now: basic artifacts that seemingly have no use and can simply be sold for money (see [items](/design/items.md)).[^analysis]
* Longer term, many artifacts will also be usable, not just sellable.[^analysis]

# Day/night cycle

* Handled as a run modifier: the player chooses day or night before entering the Zone, rather than a live in-run cycle.[^analysis]

# Economy

* For now money is only a "net-worth" score for the current character (see [progression](/design/progression.md)).[^analysis]
* Post-v0.1, money and the stash become tied to the hideout once the stalker school is unlocked (see [apprentices](/design/apprentices.md)).[^board-2025-12-04]
* Buying better equipment, healing permanent injuries, and potentially upgrading a base can come later; many artifacts will also be usable.[^analysis]

# Recommended, not yet decided

* **Stealth model**: the review recommends starting with abstracted stealth - tile-based noise values (grass = quiet, gravel = loud, water = silent), a simple detection radius (guards have X-tile vision), a binary hidden/detected state, and no gradual awareness - because full stealth (vision cones, sound propagation, AI states, hiding mechanics) is a large undertaking for a roguelike.[^analysis]

# Open questions

* **World-changing events** still need a concrete definition - for example, "every 10 minutes real-time, a Blowout occurs: all anomalies shift 1d6 tiles in a random direction, artifacts teleport to new anomaly locations, and your map markers are cleared". Frequency must be tuned so it matters without frustrating players.[^analysis]
* **Design challenges** to answer for a full GDD:[^analysis]
  1. Describe one complete run from start to finish (3-5 minutes of gameplay).
  2. List 5 anomalies with observable effect, danger to player, artifact inside it, and how the artifact changes the anomaly.
  3. Define the failure state: is death permadeath? Lose items? Lose rubles? Is detection an instant fail, combat, or an escape chance?
  4. Define the first thing a new player does in the first 30 seconds (this defines the tutorial and onboarding).
  5. Define the skill ceiling (NetHack = knowledge, Isaac = mechanical skill - what is Picnic's?).

[^analysis]: Expert design review and reuse analysis
[^board-2025-12-04]: Idea board entry, 2025-12-04
