# Systems

Specification of the implemented POC systems: the tilemap editor, the turn-based engine, item handling, the HUD, and shared UI standards. Everything here is `live` - it describes architecture that exists and works in the code today (see [lifecycle tags](/index.md#lifecycle-tags)).

# Concepts

* [Tilemap editor](/systems/tilemap-editor.md) `live` - In-game editor: modes, controls, and map serialization.
* [Turn-based engine](/systems/turn-engine.md) `live` - Turn phases, processing order, and player mechanics.
* [Ground items and inspection](/systems/ground-items.md) `live` - Placing, inspecting, picking up, and dropping items.
* [Inventory](/systems/inventory.md) `live` - Inventory UI, carry capacity, and the metal detector.
* [Bolt throwing](/systems/bolt-throwing.md) `live` - Bolt-based anomaly detection: controls, ballistics, and feedback.
* [HUD](/systems/hud.md) `live` - Turn counter, weight display, and message log.
* [UI standards](/systems/ui-standards.md) `live` - Typography, text labels, modal styling, and color palette.
