---
type: Game Design
title: Hideout upgrades and the apprentice system
description: Post-v0.1 succession design - the stalker school unlock, apprentices as backup characters with a shared hideout, deployment options, inheritance, and total-loss rules.
tags: [picnic, design, progression, permadeath, apprentices, hideout, planned]
status: stable
generated: { by: pi/glm-5.3-flash, at: 2026-10-06T04:29:00Z }
sources:
  - id: board-2026-09-30
    resource: /references/idea-board.md
    title: Idea board entry, 2026-09-30
    author: human:oleksiy
  - id: board-2025-12-04
    resource: /references/idea-board.md
    title: Idea board entry, 2025-12-04
    author: human:oleksiy
  - id: board-2025-12-03
    resource: /references/idea-board.md
    title: Idea board entry, 2025-12-03
    author: human:oleksiy
---

# Summary

The planned post-v0.1 evolution of [permadeath](/design/progression.md): hideout upgrades interact with the apprentice system so that dying costs your character's stat progress but not your legacy, until you lose everything at once. This concept records the final design as decided by the author; where it refines the original idea-board entries, this text is authoritative.[^board-2026-09-30]

# The first-death experience

At the start of the game you have no apprentice, so when you die you lose all character stat progress and all hideout progress. New players will die a lot because they do not yet know how to avoid the various dangers in the Zone. As the player learns to tell danger from opportunity, they survive, gain experience, and make enough money for bigger hideout upgrades. Around that point dying becomes very painful - you lose all of your recently accumulated progress - but the same experience attracts potential apprentices.[^board-2026-09-30]

# Unlock: the stalker school

The gateway hideout upgrade is the stalker school. Once it is unlocked:[^board-2025-12-04]

* Money and the stash become tied to the school (the base) rather than to the individual character.
* You can invest in the stalkers at your school to make them better for the inevitable moment when you must be succeeded.
* Stalkers at the school could provide passive perks (resources, money).

# Apprentices

First implementation of the apprentice feature:[^board-2025-12-03]

* Once the appropriate hideout upgrade is unlocked, you select an apprentice from a pool of local candidates - your experience attracts them.[^board-2026-09-30]
* Once selected, the apprentice stays at your hideout and has their own full stats, just like your main character - a backup character with their own stats but a shared hideout.[^board-2025-12-03]
* To work on the apprentice's stats, you can take them into the Zone with you, send them in alone, or keep them at the base.[^board-2025-12-03]

# Deployment options

* **Together**: take the apprentice into the Zone with you. Dangerous for them - they could die - but they acquire experience if they survive.[^board-2026-09-30]
* **Alone**: send the apprentice in solo. Even more risky depending on where you send them, but they gain experience faster if they survive, and they provide passive income by bringing back loot from their excursions.[^board-2026-09-30]
* **At base**: keep the apprentice at the hideout.[^board-2025-12-03]

# Succession

* If your main stalker dies at any point - or you retire them - the apprentice becomes the new main stalker, inheriting the base, money, inventory, and map. You then start playing as the apprentice; the legacy carries on.[^board-2025-12-03]
* This is the "pick which stalker to use once you die" moment the school enables; the apprentice inherits the hideout, carrying on your legacy.[^board-2025-12-04]

# Total loss

* If you die without having an apprentice, or your apprentice dies with you on a joint Zone run, you lose all progress and stats from scratch - including all hideout progress - with only the map being kept.[^board-2025-12-03]

# Relationship to v0.1 permadeath

v0.1 ships pure permadeath: a full reset of stash, contracts, money, and loadout, with map knowledge persisting (see [progression](/design/progression.md)). The stalker school and apprentice system is a designed later unlock that turns permadeath into succession once the player has something worth inheriting.[^board-2026-09-30]

[^board-2026-09-30]: Idea board entry, 2026-09-30
[^board-2025-12-04]: Idea board entry, 2025-12-04
[^board-2025-12-03]: Idea board entry, 2025-12-03 (superseded in part by the author's later refined apprentice spec, which this concept records)
