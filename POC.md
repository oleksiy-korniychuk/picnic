# POC Design Document

## Overview
Turn-based roguelike where players explore a 25x25 Zone, detect anomalies using bolts, collect items/artifacts, and extract safely. Inspired by Roadside Picnic.

## Development Priority
1. ✅ **Tilemap editor** (COMPLETE)
2. ✅ **Core turn-based engine** (COMPLETE)
3. ✅ **HUD Display** (COMPLETE)
4. ✅ **Ground Items & Inspection UI** (COMPLETE)
5. ⏳ Inventory system (IN PROGRESS)
6. Anomaly mechanics (Philosopher's Stone, Rust)
7. Win/loss conditions

## Implementation Status

### ✅ Completed: Tilemap Editor (v1.0)
**Architecture:**
- Separated terrain layer (Floor/Wall) from entity layer (Anomalies/Items/Markers)
- Dynamic grid sizing - defaults to 25x25, supports any size on load
- ECS-based: tiles and entities are proper Bevy entities with Position components

**Controls:**
- `F2` - Toggle between Running and Editing modes
- `Tab` - Cycle between Terrain, Entity, and Item placement modes
- **Terrain Mode:**
  - 1: Floor
  - 2: Wall
- **Entity Mode:**
  - 1: Gravitational Anomaly
  - 2: Philosopher's Stone
  - 3: Rust Anomaly
  - 4: Player Start
  - 5: Exit
  - 6: Lamp Post
- **Item Mode:**
  - 1: Fully Empty (artifact)
  - 2: Scrap
  - 3: Glass Jar
  - 4: Battery
- `Left Click` - Place selected terrain/entity/item
- `Right Click` - Delete entity, reset tile to Floor, or remove all items from tile
- `F3` - Quick save to `assets/maps/current.json`
- `F4` - Quick load from `assets/maps/current.json`

**Visual Feedback:**
- Gray tiles for Floor, dark gray for Walls
- Color-coded entities (purple/gold/orange for anomalies, green for start, blue for exit)
- Items.png sprite on tiles with items (only visible in Item mode in editor)
- White semi-transparent cursor highlight showing current grid position
- Minimal HUD displaying: mode, current selection (mode-specific), cursor coordinates

**Technical Implementation:**
- JSON serialization via serde for map save/load (backwards-compatible items field)
- Automatic tile/entity/item sprite reload on map load
- Keyboard-only interface (no complex UI forms)
- Grid coordinates properly convert to/from world space
- Mode-dependent key bindings (each mode starts at key 1)
- Files: `src/systems/editor.rs`, `src/systems/rendering.rs`, `src/resources/map_data.rs`, `src/components/item.rs`

### ✅ Completed: Turn-Based Engine (v1.0)
**Architecture:**
- Three-phase turn system: `PlayerTurn` (awaiting input), `WorldUpdate` (processing effects), and `InspectingItems` (paused)
- State-based scheduling using Bevy's state system
- Chained world update systems ensure deterministic execution order
- Player spawns/despawns automatically on F2 mode toggle

**Controls:**
- `WASD` - Move player in 4 directions (only during Running mode, PlayerTurn phase)
- `E` - Inspect items on current tile (transitions to InspectingItems phase)
- `ESC` - Close inspect UI (if open) or exit game
- `F2` - Toggle between Editing and Running modes
- Movement blocked by walls (no turn consumed if invalid)

**Turn Processing Order:**
1. Player inputs movement (WASD) or inspection (E) during PlayerTurn
   - Movement: validates → updates position → advances to WorldUpdate
   - Inspection: opens modal UI → transitions to InspectingItems (pauses game)
2. InspectingItems phase (optional):
   - Game paused, turn does not advance
   - ESC closes UI → returns to PlayerTurn
3. WorldUpdate phase (chained systems):
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
- Resources: `TurnPhase` state (PlayerTurn, WorldUpdate, InspectingItems), `TurnCounter` resource
- Components: `Player` marker, `GravitationalAnomalyTimer(u32)`, `GroundItems`
- Game states: `Running` and `Editing` (Paused removed)
- Contextual ESC handling (closes inspect UI when open, otherwise exits game)
- Files: `src/systems/player.rs`, `src/systems/turn_based_input.rs`, `src/systems/turn_processor.rs`, `src/resources/turn_state.rs`, `src/systems/inspect_ui.rs`, `src/systems/ground_items.rs`

**What's NOT Yet Implemented:**
- Inventory system (pickup/drop from ground)
- Bolt throwing
- Other anomaly types (Philosopher's Stone, Rust)
- Win condition (extraction)
- Full game reset on death

### ✅ Completed: Ground Items & Inspection UI (v1.0)
**Architecture:**
- Item data structure with name, weight, and optional value
- `GroundItems` component attached to tile positions
- Items rendered using `Items.png` sprite (60% tile size)
- Inspection UI is modal and pauses gameplay
- Three-mode editor: Terrain, Entity, and Item placement

**Editor Integration:**
- New "Item" mode in editor (Tab to cycle: Terrain → Entity → Item)
- Mode-dependent key bindings (each mode starts at key 1)
- **Item Mode Keys:**
  - 1: Fully Empty (artifact, 100 weight, 200 value)
  - 2: Scrap (10 weight, 5 value)
  - 3: Glass Jar (5 weight, 2 value)
  - 4: Battery (3 weight, 3 value)
- Left-click to place items (multiple items can stack on same tile)
- Right-click to remove all items from tile
- Items saved/loaded with map (F3/F4)

**Player Interaction:**
- `E` key to inspect items when standing on tile with items
- Modal UI displays scrollable list of items with name/weight/value
- `ESC` closes inspect UI (contextually aware - doesn't exit game)
- No pickup yet (awaits full inventory system)

**Visual Feedback:**
- Items.png sprite appears on tiles with items (only in Running mode)
- Sprite renders above entities but below player (z=1)
- Inspector modal: semi-transparent overlay with bordered panel
- Item list shows formatted text: "1. ItemName (Weight: X, Value: Y)"

**Technical Implementation:**
- Components: `Item`, `GroundItems`, `GroundItemSprite`, `InspectUiRoot`
- Turn phase: `TurnPhase::InspectingItems` (pauses turn flow)
- Files: `src/components/item.rs`, `src/systems/ground_items.rs`, `src/systems/inspect_ui.rs`
- Map serialization: Backwards-compatible with `#[serde(default)]`
- Items persisted in JSON as `items: Vec<PlacedGroundItems>`

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
- **Three Modes**: Terrain, Entity, Item (Tab to cycle)
- **Mode-Dependent Keys**: Each mode starts at key 1 (no shared number row)
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

**Ground Items** ✅ COMPLETE:
- Items can be placed on ground tiles via editor (Item mode)
- Items.png sprite renders on tiles with items (Running mode only)
- `E` key to inspect items on current tile
- Modal UI shows item details (name, weight, value)
- Items persist in map JSON (backwards-compatible serialization)

**Items** (Implemented in editor, not yet in inventory):
| Item | Weight | Value | Properties |
|------|--------|-------|------------|
| Bolt | 1 | - | Throwable, starting: 10 (NOT YET IMPLEMENTED) |
| Fully Empty | 100 | 200 | Artifact ✅ |
| Metal Detector | 50 | - | Tool, beeps within 2 tiles, metal (NOT YET IMPLEMENTED) |
| Scrap | 10 | 5 | Metal ✅ |
| Glass Jar | 5 | 2 | Non-metal ✅ |
| Battery | 3 | 3 | Non-metal ✅ |
| Rust Slag | 5 | 0 | Byproduct, metal (NOT YET IMPLEMENTED) |
| Backpack | - | - | Tool, can be dropped entirely (NOT YET IMPLEMENTED) |

**Carry System** (NOT YET IMPLEMENTED):
- Normal capacity: 250
- Gravitational anomaly: 125 (halved)
- Starting loadout: 10 bolts, metal detector, backpack

**Inventory UI** (NOT YET IMPLEMENTED):
- Roguelike list (arrow keys to select, list should be scrollable, space to drop)
- Display: item name, weight, value (where applicable)
- Actions: Pick up item, Drop individual item, drop full backpack

**Pickup/Drop Mechanics** (NOT YET IMPLEMENTED):
- Pickup items from current tile
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

### 6. Ground Items ✅ COMPLETE
- **Visual**: Items.png sprite on tile (60% tile size, z=1)
- **Interaction**: Player stands on tile → press `E` → modal UI shows item list
- **Inspection**: Scrollable list with item name, weight, and value
- **Note**: Pickup/inventory integration not yet implemented

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
