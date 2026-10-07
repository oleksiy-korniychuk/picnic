---
type: Game Design
title: Contracts
description: Contract pool generation, example contracts with rewards, and the turn-in flow that pays a bonus over selling.
tags: [picnic, design, contracts, economy, planned]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: progression
    resource: /references/source-docs/v0.1_progression.md
    title: Progression System Design (v0.1)
    author: human:oleksiy
    last_modified: 2025-12-06T05:40:56Z
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
---

# Pool generation

* 5 random contracts are available at any time.[^progression]
* Generated when the player first opens the Contracts screen on the current stalker.[^progression]
* A new random contract is generated each time the player marks a contract as active, always maintaining 5 available.[^progression]
* The pool resets when the current stalker dies.[^progression]
* Up to 3 contracts can be active simultaneously.[^progression]

# Contract types

| Contract | Description | Reward | Bonus vs selling |
|----------|-------------|--------|------------------|
| Battery Runner | Retrieve 5 batteries | 25 rubles | +10 (vs 15) |
| Scrap Haul | Bring back 100 weight of scrap | 100 rubles | +50 (vs 50) |
| Relic Hunt | Collect 1 Fully Empty | 300 rubles | +100 (vs 200) |
| Detector Recovery | Retrieve 2 metal detectors | 150 rubles | +50 (vs 100) |

Reward formula: contract rewards should be ~1.3-1.5x the item's sell value to incentivize completion.[^progression]

# Mechanics

* **Selection**: choose up to 3 active contracts any time in the [base hub](/design/progression.md).[^progression]
* **Turn-in**: in the Contracts screen, select an active contract and press E:[^progression]
  * Validates the player has the required items in the run inventory or the stash.
  * Consumes the items from either inventory.
  * Awards money and marks the contract complete.
  * The contract is removed from the active list (its slot opens, and a new contract replenishes the available pool).
  * Message log: "Contract complete! Earned <reward> rubles for completing: <description>".

# In-run presentation

* The mission briefing screen on zone entry shows the active contracts (for example "Collect 3 artifacts of value 100 or greater"); press E to accept and begin.[^poc]
* The extraction screen shows contract completion status with `[COMPLETE]`/`[FAILED]` markers, styled per the [UI standards](/systems/ui-standards.md).[^poc]

[^progression]: Progression System Design (v0.1)
[^poc]: POC Design Document
