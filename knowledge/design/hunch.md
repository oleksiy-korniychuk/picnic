---
type: Game Design
title: Hunch system
description: Planned passive anomaly perception via reliable perception radii (good and peripheral) instead of probability checks, with radii growing with character experience.
tags: [picnic, design, anomalies, perception, hunches, planned]
status: draft
generated: { by: pi/glm-5.3-flash, at: 2026-10-06T04:29:00Z }
sources:
  - id: board-2026-09-30
    resource: /references/idea-board.md
    title: Idea board entry, 2026-09-30
    author: human:oleksiy
  - id: board-2025-11-30
    resource: /references/idea-board.md
    title: Idea board entry, 2025-11-30
    author: human:oleksiy
---

# Direction

The hunch system is the planned passive layer of [anomaly perception](/design/anomalies.md), complementing active [bolt detection](/systems/bolt-throwing.md). The game is materializing to be about exploration and ultra-hard play without deep player knowledge, so perception should be reliable - a system the player can rely on - leaving the uncertainty to world knowledge (anomalies, the map, items, events, etc.).[^board-2026-09-30]

# Radius-based, not probability-based

The original sketch rolled a check when the character was adjacent to an invisible anomaly, with success and partial-success outcomes. That probability-based model is superseded: the hunch system should not be probability based but instead radius based, with a **"good perception" radius** and a **"peripheral perception" radius**. This gives a much more reliable perception system the player can rely on.[^board-2026-09-30]

# Perception radii

* **Good perception radius**: within this radius, sensory hints are dependable.
* **Peripheral perception radius**: beyond the good radius, hints are weaker and peripheral.
* Both radii can grow as a character's experience grows over time.[^board-2026-09-30]

# Sensory hints

The form the hints take comes from the original sketch (message text illustrative): for example "The hairs on your arm stand up" (Electro) or "The air smells like ozone" (Energy), with a vaguer fallback like "You feel a sense of dread." Under the radius model, the good radius yields the specific hint and the peripheral radius the vaguer one.[^board-2025-11-30]

The hunch system assumes anomalies are not visible during normal play - the POC's purple overlays are a debugging aid (see [anomalies](/design/anomalies.md)).[^board-2025-11-30]

# Open questions

* Whether the originally sketched passive skills ("Acoustics," "Thermodynamics," "Intuition") survive alongside the experience-driven radii, or whether experience growth alone drives the radii.[^board-2025-11-30]
* Which sensory message maps to which anomaly type, and what the peripheral-tier message is per type.
* Exact radius sizes and growth rates.

Related uncommitted idea: [grid-distortion tile VFX](/design/future-ideas.md), from the same board entry.

[^board-2026-09-30]: Idea board entry, 2026-09-30
[^board-2025-11-30]: Idea board entry, 2025-11-30
