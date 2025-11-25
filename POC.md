# POC Design Document

## Overview
Turn-based roguelike where players explore a 25x25 Zone, detect anomalies using bolts, collect items/artifacts, and extract safely. Inspired by Roadside Picnic.

## Development Priority
1. ✅ **Tilemap editor** (COMPLETE)
2. ⏳ Core turn-based engine (IN PROGRESS)
3. Movement & item systems
4. Anomaly mechanics
5. Win/loss conditions

## Implementation Status

### ✅ Completed: Tilemap Editor (v1.0)
**Architecture:**
- Separated terrain layer (Floor/Wall) from entity layer (Anomalies/Items/Markers)
- Dynamic grid sizing - defaults to 25x25, supports any size on load
- ECS-based: tiles and entities are proper Bevy entities with Position components

**Controls:**
- `F2` - Toggle between Running and Editing modes
- `Tab` - Switch between Terrain and Entity placement modes
- `1-2` - Select terrain (1=Floor, 2=Wall)
- `3-9` - Select entities:
  - 3: Gravitational Anomaly
  - 4: Philosopher's Stone
  - 5: Rust Anomaly
  - 6: Player Start
  - 7: Exit
  - 8: Lamp Post
  - 9: Fully Empty (artifact)
- `Left Click` - Place selected terrain/entity
- `Right Click` - Delete entity or reset tile to Floor
- `F3` - Quick save to `assets/maps/current.json`
- `F4` - Quick load from `assets/maps/current.json`

**Visual Feedback:**
- Gray tiles for Floor, dark gray for Walls
- Color-coded entities (purple/gold/orange for anomalies, green for start, blue for exit)
- White semi-transparent cursor highlight showing current grid position
- Minimal HUD displaying: mode, current selection, cursor coordinates

**Technical Implementation:**
- JSON serialization via serde for map save/load
- Automatic tile/entity sprite reload on map load
- Keyboard-only interface (no complex UI forms)
- Grid coordinates properly convert to/from world space
- Files: `src/systems/editor.rs`, `src/systems/rendering.rs`, `src/resources/map_data.rs`

## Core Systems

### 1. Tilemap Editor ✅ COMPLETE
- **Access**: In-game via hotkey (F2)
- **Functionality**: Load, edit, save dynamic-sized tile maps (default 25x25)
- **Placeable Elements**: Walls, floors, anomalies, items, player start/exit, structures (lamp posts)
- **Format**: JSON files (serde serialization)
- **Implementation**: See "Implementation Status" section above for full details

### 2. Turn-Based Engine
- **Input**: 4-directional movement (WASD keyboard)
- **Turn Structure**: Player action → World update → Player action
- **Processing Order**:
  1. Player performs action
  2. Check Gravitational anomaly pull (adjacent tiles)
  3. Process anomaly end-of-turn effects
  4. Update Gravitational anomaly countdown
  5. Check death condition
  6. Return to player input

### 3. Item & Inventory System

**Items**:
| Item | Weight | Value | Properties |
|------|--------|-------|------------|
| Bolt | 1 | - | Throwable, starting: 10 |
| Fully Empty | 100 | 200 | Artifact |
| Metal Detector | 50 | - | Tool, beeps within 2 tiles, metal |
| Scrap | 10 | 5 | Metal |
| Glass Jar | 5 | 2 | Non-metal |
| Battery | 3 | 3 | Non-metal |
| Rust Slag | 5 | 0 | Byproduct, metal |
| Backpack | - | - | Tool, can be dropped entirely |

**Carry System**:
- Normal capacity: 250
- Gravitational anomaly: 125 (halved)
- Starting loadout: 10 bolts, metal detector, backpack

**UI**:
- Roguelike list (1-9 hotkeys, mouse click, arrow keys + space)
- Display: item name, weight, value (where applicable)
- Actions: Drop individual item, drop full backpack

**Drop Mechanics**:
- Items drop on tile in front of player
- If blocked, drop on current tile

### 4. Anomaly System

**Visual**: All anomalies appear as faint purple overlay (indistinguishable)

**Types**:

**Gravitational Anomaly**:
- Pull: Player within 1 tile pulled in during world update
- Effect: Carry capacity reduced to 125
- Death: 5 turns inside anomaly = crushed
- Text: "You feel as if you weigh a thousand pounds. Every fiber in your body strains and creaks under the weight."

**Philosopher's Stone**:
- Trigger: End of turn, if ground items present
- Effect: Destroys 1 random ground item, replaces with equal/lesser value item, 5% chance → Fully Empty
- Text: Descriptive transformation message

**The Rust**:
- Trigger: End of turn, if metal items present (ground OR player inventory on tile)
- Effect: Destroys 1 random metal item → Rust Slag
- Text: "The [item] on the ground begins to rust rapidly before your very eyes. In an instant, it melts into a rusty glob."

**Text Logs**: All anomaly effects generate atmospheric text when player nearby

### 5. Bolt Detection System
- **Range**: 5 tiles straight line (4 directions)
- **Animation**: 0.5 second flight, tile-by-tile with fading trail
- **Collision**: Stops at solid objects or anomalies
- **Feedback**: Text description of anomaly interaction

### 6. Ground Items
- **Visual**: Generic "items" icon on tile
- **Interaction**: Player stands on tile → "inspect" action reveals item list → pick up individual items

### 7. Metal Detector
- **Range**: 2 tiles in any direction
- **Feedback**: UI icon pulses when items detected
- **No audio** (POC)

### 8. HUD Display
- Current weight / Max weight
- Turn counter
- Message log (anomaly effects, actions)
- Metal detector status icon (pulses when active)

### 9. Win/Loss Conditions

**Win**: Return to entrance/exit with items → Complete contract (objective: extract with artifacts)

**Loss**: Death in Gravitational anomaly (5 turns) → Full reset (permadeath)

## Technical Notes
- Map size: 25x25 tiles (handcrafted via editor)
- Anomalies visible for debugging (purple overlay)
- 4-directional movement only
- No combat, no audio (POC)
- Traditional roguelike controls/UI patterns

## Architecture Refactor
Existing codebase is real-time ECS. POC requires:
- Turn-based game loop (action queue system)
- Event-driven anomaly processing
- Inventory/weight component system
- Text log system
- Tile-based collision and item management

Reusable from existing code (~60%):
- Camera, grid system, coordinates, pathfinding
- Sprite rendering, input framework, UI panels
- Spatial indexing, parent-child entities
