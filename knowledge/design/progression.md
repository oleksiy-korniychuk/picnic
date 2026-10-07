---
type: Game Design
title: Progression (v0.1)
description: Permadeath rules, the base hub (stash management and contracts screens), the money system, and item flow between runs.
tags: [picnic, design, progression, permadeath, in-progress]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: progression
    resource: /references/source-docs/v0.1_progression.md
    title: Progression System Design (v0.1)
    author: human:oleksiy
    last_modified: 2025-12-06T05:40:56Z
  - id: board-2025-12-04
    resource: /references/idea-board.md
    title: Idea board entry, 2025-12-04
    author: human:oleksiy
---

# Overview

Roguelike progression with permadeath. Each stalker has one life - death resets stash, contracts, and money, while map knowledge persists. Players manage a base stash, select contracts, prepare loadouts, and choose between selling items for base value, turning them in for contract bonuses, or keeping them for future runs.[^progression]

# Base hub

The base hub is two modal screens.[^progression]

## Stash Management (default screen)

* **Layout**: left panel = run inventory (the character's current loadout, 250 weight capacity); right panel = stash (persistent storage, 1000 weight capacity).[^progression]
* **Controls**: arrow keys navigate between panels and items; Enter/E selects an item and moves it to the other inventory; S sells the selected item (from either inventory) for base value; Tab switches to the Contracts screen; Space enters the Zone with the current run inventory contents; ESC quits the game.[^progression]
* **Display**: each item shows name, weight, and value (if applicable); weight totals ("Run Inventory: 60/250" and "Stash: 320/1000"); current money displayed at top.[^progression]
* **Weight rules**: no weight restrictions inside the base hub (items move freely between inventories); if the run inventory exceeds 250 capacity when entering the Zone, the player cannot move until weight is reduced - the existing [overweight logic](/design/items.md) handles this.[^progression]

## Contracts screen

See [contracts](/design/contracts.md).

# Money system

* **Run money** is the current stalker's earnings: starts at 0 each new stalker, earned by selling items and completing contracts, displayed prominently in the base hub, and resets on death. Pure score/achievement (no spending yet).[^progression]
* Post-v0.1, once the [stalker school](/design/apprentices.md) hideout upgrade is unlocked, money and the stash become tied to the school (the base) rather than the character, and are inherited on succession.[^board-2025-12-04]

# Permadeath rules

When the player dies in the Zone:[^progression]

* **Lost forever**: run inventory (everything carried at the time of death), base stash (all stored items), active contracts, run money.
* **Kept**: map knowledge - explored terrain persists across stalkers; only player-owned progression resets.[^board-2025-12-04] The Zone itself is restored from the saved map state, which is equivalent in v0.1 because the map is static, and diverges once exploration fog or world-changing events exist.
* **Fresh start**: a new stalker spawns; run inventory gets the starter loadout (10 bolts + metal detector); the stash is empty (1000 capacity); no active contracts; a new contract pool (5 random); money = 0.

Map persistence supersedes the original v0.1 document, which listed map state among the things lost on death.[^progression] Post-v0.1, permadeath evolves into succession via the [stalker school and apprentice system](/design/apprentices.md).

# Item flow

* **First run (new stalker)**: run inventory = 10 bolts (10 weight) + metal detector (50 weight) = 60 total; stash empty.[^progression]
* **Post-extraction**: return to the base hub with everything extracted from the Zone, then manually manage items:[^progression]
  * Move items run inventory → stash (if space is available) and stash → run inventory (to prepare for the next run).
  * Sell items from either inventory (S key).
  * Switch to the Contracts tab and turn in items for active contracts (E key); select new contracts if slots are available.
  * Ensure the run inventory has the desired loadout and press Space to enter the Zone.
* **Pre-run loadout prep**: the run inventory IS the loadout; items are moved from the stash manually. The player can enter the Zone with an empty run inventory if desired (hardcore mode).[^progression]

# Implementation status (base hub UI)

The base hub UI is complete as visual structure:[^progression]

* `InBaseHub` state added to the `GameState` enum; `BaseHubMode` state with two modes: `StashManagement` (default) and `Contracts`; two full-screen modal UIs following the [UI standards](/systems/ui-standards.md) (font sizes 24.0 titles / 16.0 subtitles and help / 18.0 body; panels 700-900px wide, padding 40px, row gap 20px; z-index 100).
* Stash Management screen: two-panel layout, money display placeholder ("0 Rubles"), weight totals for both inventories, placeholder items for visual testing, help text ("Tab - Contracts | Space - Enter Zone | ESC - Quit Game").
* Contracts screen: Active Contracts section (0/3 placeholder), Available Contracts section (5 placeholder contracts), help text ("Tab - Stash Management | E - Select/Turn In | ESC - Quit Game").
* State transitions: exit tile → ExitingZone → E → InBaseHub (Stash Management mode); Stash Management ↔ Contracts via Tab; Stash Management → Space → Running (re-enter the Zone on the PlayerStart tile); ESC in the base hub quits the game (temporary, until proper implementation).
* Files: created `src/systems/base_hub_ui.rs`; modified `src/resources/game_state.rs`, `src/systems/contract_ui.rs`, `src/systems/editor.rs`, `src/systems/rendering.rs`, `src/main.rs`.[^progression]

Remaining v0.1 work is tracked in the [roadmap](/roadmap/v0.1-remaining.md).[^progression]

[^progression]: Progression System Design (v0.1)
[^board-2025-12-04]: Idea board entry, 2025-12-04
