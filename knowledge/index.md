---
okf_version: "0.2"
---

# Picnic Knowledge Bundle

Knowledge for **Picnic**, a roguelike game inspired by *Roadside Picnic*, built with Rust and the Bevy game engine (v0.16.0). Concepts are grouped by concern: project overview, game design, implemented systems, and roadmap. The repository markdown documents this bundle was converted from live verbatim under [references/source-docs/](references/source-docs/).

# Lifecycle tags

Every work-describing document (game design, system spec, roadmap, status) carries exactly one **lifecycle tag** in its `tags` frontmatter, telling you from the bundle alone whether what the document describes is real in the code or still an idea. Meta documents (indexes, logs, overviews, decision records, analyses) carry none.

| Tag | Meaning |
|-----|---------|
| `live` | Describes a feature or architecture that exists and works in the code today |
| `in-progress` | Describes work that is partly live, partly still planned (mixed document) |
| `planned` | Describes designed/specified work with nothing live in the code yet |
| `idea` | Aspirational direction or brainstorm - not scheduled concrete work |
| `reference` | Verbatim source mirror or external spec (not a lifecycle of work) |
| `snapshot` | Time-bound status snapshot; superseded by newer snapshots |

Note: the separate `status:` frontmatter field (`draft`/`stable`) is *editorial* - how settled the writing is - and says nothing about implementation.

Query the bundle from the repo root (the pattern matches the tag as a whole YAML list element, so `idea-board` does not match an `idea` query):

```sh
# what is done (features/architecture working in the code)
grep -rlE '^tags:.*(, |\[)live(, |\])' knowledge
# what is ongoing / mixed
grep -rlE '^tags:.*(, |\[)in-progress(, |\])' knowledge
# what is planned but not built
grep -rlE '^tags:.*(, |\[)planned(, |\])' knowledge
# what is aspiration / brainstorm (not scheduled)
grep -rlE '^tags:.*(, |\[)idea(, |\])' knowledge
```

Listings below are annotated with the lifecycle tag in backticks.

# Project

* [Picnic overview](/overview.md) - What the game is, the stack, and current status.
* [Code reuse analysis](/reuse-analysis.md) - What carries over from the real-time codebase and what the POC refactor requires.

# Status

* [Current state snapshot](/status/current-state.md) `snapshot` - Point-in-time left-off point (fc71ea6 + uncommitted bug fixes): what is implemented, the two original bugs fixed and verified, and next steps.

# Game Design

* [Design decisions](/design/decisions.md) - Resolved answers to the expert design review, plus open questions.
* [Core loop](/design/core-loop.md) `live` - Turn phases, run structure, and win/loss conditions.
* [Anomalies](/design/anomalies.md) `live` - The three POC anomalies, their mechanics, and bolt-based detection.
* [Items and carry weight](/design/items.md) `in-progress` - The item catalog, weights and values, and carry capacity rules.
* [Contracts](/design/contracts.md) `planned` - Contract pool generation, types, rewards, and turn-in flow.
* [Progression v0.1](/design/progression.md) `in-progress` - Permadeath, the base hub, money, and item flow between runs.
* [Apprentice system](/design/apprentices.md) `planned` - Post-v0.1 succession: the stalker school unlock, apprentices, deployment, and inheritance.
* [Hunch system](/design/hunch.md) `planned` - Planned passive anomaly perception via reliable perception radii.
* [Dave the Diver inspiration](/design/dave-the-diver.md) `idea` - Multi-location home base, story beats as checkpoints, and mechanic evolution.
* [Future ideas](/design/future-ideas.md) `idea` - Uncommitted idea-board material and the disposition of every reviewed entry.

# Systems

* [Tilemap editor](/systems/tilemap-editor.md) `live` - In-game editor: modes, controls, and map serialization.
* [Turn-based engine](/systems/turn-engine.md) `live` - Turn phases, processing order, and player mechanics.
* [Ground items and inspection](/systems/ground-items.md) `live` - Placing, inspecting, picking up, and dropping items.
* [Inventory](/systems/inventory.md) `live` - Inventory UI, carry capacity, and the metal detector.
* [Bolt throwing](/systems/bolt-throwing.md) `live` - Bolt-based anomaly detection: controls, ballistics, and feedback.
* [HUD](/systems/hud.md) `live` - Turn counter, weight display, and message log.
* [UI standards](/systems/ui-standards.md) `live` - Typography, text labels, modal styling, and color palette.

# Roadmap

* [v0.1 remaining work](/roadmap/v0.1-remaining.md) `planned` - The work items left to complete the v0.1 progression milestone.

# References

* [OKF v0.2 specification](/references/okf-spec-v0.2.md) - Native copy of the Open Knowledge Format spec this bundle targets, sourced from GoogleCloudPlatform/open-knowledge-format.
* [Idea board](/references/idea-board.md) - Verbatim entries from the author's idea board (2025-11-14 through 2026-09-30).
* [Source documents](references/source-docs/) - The four repository markdown docs this bundle was converted from, kept verbatim.
