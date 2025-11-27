# POC Design Document

## Overview
Turn-based roguelike where players explore a 25x25 Zone, detect anomalies using bolts, collect items/artifacts, and extract safely. Inspired by Roadside Picnic.

## Development Priority
1. ✅ **Tilemap editor** (COMPLETE)
2. ✅ **Core turn-based engine** (COMPLETE)
3. ✅ **HUD Display** (COMPLETE)
4. ⏳ Item & inventory systems (IN PROGRESS)
5. Anomaly mechanics (Philosopher's Stone, Rust)
6. Win/loss conditions

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

### ✅ Completed: Turn-Based Engine (v1.0)
**Architecture:**
- Two-phase turn system: `PlayerTurn` (awaiting input) and `WorldUpdate` (processing effects)
- State-based scheduling using Bevy's state system
- Chained world update systems ensure deterministic execution order
- Player spawns/despawns automatically on F2 mode toggle

**Controls:**
- `WASD` - Move player in 4 directions (only during Running mode, PlayerTurn phase)
- `F2` - Toggle between Editing and Running modes
- Movement blocked by walls (no turn consumed if invalid)

**Turn Processing Order:**
1. Player inputs movement (WASD) → validates → updates position → advances to WorldUpdate
2. WorldUpdate phase (chained systems):
   - Gravitational anomaly pull (adjacent tiles)
   - Anomaly effects (placeholder for Philosopher's Stone, Rust)
   - Timer updates (gravitational anomaly countdown)
   - Death check (timer reaches 0)
   - Turn counter increment
   - Transition back to PlayerTurn

**Player Mechanics:**
- Spawns at `PlayerStart` marker when entering Running mode
- Visual representation: Red.png sprite (80% tile size)
- Camera auto-follows player position
- Logical position (Position component) separate from visual (Transform)
- Camera panning disabled during Running mode

**Gravitational Anomaly (Basic Implementation):**
- Pulls player when adjacent (1 tile away, 4-directional) AND player doesn't have a timer
- Player pulled 1 tile toward anomaly during WorldUpdate
- Timer starts at 5 turns when player enters anomaly
- Timer decrements each turn player remains **within range** (on anomaly OR adjacent)
- Timer only removed when player escapes to **safe distance** (>1 tile away)
- Player dies (returns to Editing) when timer reaches 0
- Escape requires minimum 2 turns: off the anomaly tile → out of pull range

**Technical Implementation:**
- Resources: `TurnPhase` state, `TurnCounter` resource
- Components: `Player` marker, `GravitationalAnomalyTimer(u32)`
- Game states reduced to: `Running` and `Editing` (Paused removed)
- Files: `src/systems/player.rs`, `src/systems/turn_based_input.rs`, `src/systems/turn_processor.rs`, `src/resources/turn_state.rs`

**What's NOT Yet Implemented:**
- Inventory system
- Item pickup/drop
- Bolt throwing
- Other anomaly types (Philosopher's Stone, Rust)
- Win condition (extraction)
- Full game reset on death

### ✅ Completed: HUD Display (v1.0)
**Architecture:**
- Message log resource stores last 5 messages
- UI components spawned/despawned with Running mode
- Change detection for efficient updates

**Display Elements:**
- Turn counter (updates each turn)
- Weight display (placeholder: 0/250 until inventory implemented)
- Message log (last 5 messages, oldest to newest from top to bottom)
- Positioned at bottom of screen with semi-transparent background

**Messages:**
- "You enter the Zone..." (on spawn)
- "Gravitational anomaly pulls you in!" (when pulled)
- "Immense pressure... 5 turns to escape!" (timer starts)
- "Crushing pressure! X turns left!" (each turn in range)
- "You break free from the anomaly!" (escaped)
- "You are crushed to death!" (death)

**Visual Styling:**
- Semi-transparent black background (rgba 0,0,0,0.8)
- White text, monospace font
- Messages fade slightly with age (newest brightest)
- Stats bar at top, message log below

**Technical Implementation:**
- Resource: `MessageLog` (VecDeque with max 5 messages)
- Components: `GameHudRoot`, `TurnCounterText`, `WeightText`, `MessageLogText`
- Files: `src/resources/message_log.rs`, `src/systems/hud.rs`
- Only visible during Running mode

## Core Systems

### 1. Tilemap Editor ✅ COMPLETE
- **Access**: In-game via hotkey (F2)
- **Functionality**: Load, edit, save dynamic-sized tile maps (default 25x25)
- **Placeable Elements**: Walls, floors, anomalies, items, player start/exit, structures (lamp posts)
- **Format**: JSON files (serde serialization)
- **Implementation**: See "Implementation Status" section above for full details

### 2. Turn-Based Engine ✅ COMPLETE
- **Input**: 4-directional movement (WASD keyboard)
- **Turn Structure**: Player action → World update → Player action
- **Processing Order**:
  1. Player performs action (validates wall collision)
  2. Check Gravitational anomaly pull (adjacent tiles)
  3. Process anomaly end-of-turn effects (stub)
  4. Update Gravitational anomaly countdown
  5. Check death condition
  6. Increment turn counter
  7. Return to player input
- **Implementation**: See "Implementation Status" section above for full details

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

### 8. HUD Display ✅ COMPLETE
- Current weight / Max weight (placeholder: 0/250)
- Turn counter
- Message log (last 5 messages: anomaly effects, death, escape)
- Metal detector status icon (NOT YET IMPLEMENTED)
- **Implementation**: See "Implementation Status" section above for full details

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
