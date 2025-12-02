# POC Design Document

## Overview
Turn-based roguelike where players explore a 25x25 Zone, detect anomalies using bolts, collect items/artifacts, and extract safely. Inspired by Roadside Picnic.

## Development Priority
1. ✅ **Tilemap editor** (COMPLETE)
2. ✅ **Core turn-based engine** (COMPLETE)
3. ✅ **HUD Display** (COMPLETE)
4. ✅ **Ground Items & Inspection UI** (COMPLETE)
5. ✅ **Inventory system** (COMPLETE)
6. ✅ **Anomaly mechanics** (COMPLETE - Philosopher's Stone, Rust, Gravitational)
7. ✅ **Bolt throwing system** (COMPLETE)
8. ✅ **Win/loss conditions** (COMPLETE)

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
  - 5: Bolt
  - 6: Metal Detector
  - 7: Rust Slag
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
- `E` - Inspect items on current tile (transitions to InspectingItems phase), pickup selected item from inspect UI
- `Tab` - Open inventory UI (transitions to ViewingInventory phase)
- `D` - Drop selected item from inventory (places on current tile)
- `ESC` - Close inspect/inventory UI (consumes 1 turn) or exit game
- `F2` - Toggle between Editing and Running modes
- Movement blocked by walls or being overweight (no turn consumed if invalid)

**Turn Processing Order:**
1. Player inputs movement (WASD), inspection (E), or inventory (Tab) during PlayerTurn
   - Movement: validates weight → updates position → advances to WorldUpdate (1 turn)
   - Inspection: opens modal UI → transitions to InspectingItems (pauses game, no turn yet)
   - Inventory: opens modal UI → transitions to ViewingInventory (pauses game, no turn yet)
2. InspectingItems phase (optional):
   - Game paused, turn does not advance while menu open
   - Arrow keys navigate item list, E picks up selected item
   - ESC closes UI → advances to WorldUpdate (1 turn consumed)
3. ViewingInventory phase (optional):
   - Game paused, turn does not advance while menu open
   - Arrow keys navigate inventory, D drops selected item
   - ESC closes UI → advances to WorldUpdate (1 turn consumed)
4. WorldUpdate phase (chained systems):
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
- Resources: `TurnPhase` state (PlayerTurn, WorldUpdate, InspectingItems, ViewingInventory), `TurnCounter`, `CarryCapacity`
- Components: `Player` marker, `GravitationalAnomalyTimer(u32)`, `GroundItems`, `Inventory`
- Game states: `Running` and `Editing` (Paused removed)
- Contextual ESC/Tab handling (closes inspect/inventory UI when open, otherwise exits game)
- Files: `src/systems/player.rs`, `src/systems/turn_based_input.rs`, `src/systems/turn_processor.rs`, `src/resources/turn_state.rs`, `src/systems/inspect_ui.rs`, `src/systems/ground_items.rs`, `src/systems/inventory_ui.rs`, `src/components/inventory.rs`, `src/systems/metal_detector.rs`

**All Features Complete** ✅

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
- Arrow keys navigate item list, `E` picks up selected item
- `ESC` closes inspect UI (contextually aware - doesn't exit game)
- Pickup adds item to inventory and removes from ground
- Ground entity despawns when last item picked up

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
- Weight display (actual inventory weight / capacity, red text if overweight)
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

### ✅ Completed: Inventory System (v1.0)
**Architecture:**
- Unlimited inventory capacity (items always picked up)
- Weight-based movement restriction (blocked if over capacity)
- Capacity varies by context: 250 normal, 125 in gravity
- Starting loadout: 10 Bolts (10 weight) + Metal Detector (50 weight) = 60 total

**Inventory UI:**
- `Tab` key opens modal inventory UI (ViewingInventory phase)
- Scrollable list showing all carried items
- Arrow keys navigate selection
- `D` key drops selected item
- `ESC` closes UI and returns to PlayerTurn
- Weight display: "Current/Max" in red if overweight

**Pickup System:**
- Integrated into inspect UI (arrow keys + E on selected item)
- Items added to inventory component
- Removed from GroundItems component
- Ground entity despawned when empty
- Message log confirms pickup

**Drop System:**
- `D` key drops selected item from inventory
- Item always placed on player's current tile
- Creates new GroundItems entity or adds to existing
- Message log confirms drop

**Metal Detector:**
- Visual indicator in top-right corner: "⚠ METAL DETECTED"
- Only active if Metal Detector in inventory
- Scans 2-tile radius (Manhattan distance: dx + dy <= 2)
- Detects metal items on ground (is_metal field)
- Indicator shows/hides based on detection

**Movement Restriction:**
- Weight checked before each WASD movement
- Blocked if inventory weight > capacity
- Message: "You're carrying too much weight to move!"
- No turn consumed when blocked
- Capacity halved (125) when in gravitational anomaly

**Technical Implementation:**
- Resources: `CarryCapacity` (normal: 250, in_gravity: 125)
- Components: `Inventory` (Vec<Item>), `InventorySelection`, `MetalDetectorIndicator`
- Turn phases: `ViewingInventory` (pauses game, allows inventory management)
- Files: `src/components/inventory.rs`, `src/systems/inventory_ui.rs`, `src/systems/metal_detector.rs`
- All items have `is_metal` field for metal detector and Rust anomaly

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

### 3. Item & Inventory System ✅ COMPLETE

**Ground Items** ✅ COMPLETE:
- Items can be placed on ground tiles via editor (Item mode)
- Items.png sprite renders on tiles with items (Running mode only)
- `E` key to inspect items on current tile
- Modal UI shows item details (name, weight, value)
- Items persist in map JSON (backwards-compatible serialization)

**Items** ✅ ALL IMPLEMENTED:
| Item | Weight | Value | Properties |
|------|--------|-------|------------|
| Bolt | 1 | 1 | Throwable (not yet), starting: 10 ✅ |
| Fully Empty | 100 | 200 | Artifact ✅ |
| Metal Detector | 50 | - | Tool, beeps within 2 tiles, metal ✅ |
| Scrap | 10 | 5 | Metal ✅ |
| Glass Jar | 5 | 2 | Non-metal ✅ |
| Battery | 3 | 3 | Non-metal ✅ |
| Rust Slag | 5 | 0 | Byproduct, metal ✅ |

**Carry System** ✅ COMPLETE:
- Normal capacity: 250
- Gravitational anomaly: 125 (halved)
- Starting loadout: 10 bolts (10 weight), metal detector (50 weight)
- Movement blocked when over capacity
- Inventory is "unlimited" but player cannot move when overweight

**Inventory UI** ✅ COMPLETE:
- `Tab` key opens modal inventory
- Arrow keys to navigate, scrollable list
- `D` to drop selected item
- Display: item name, weight, value (where applicable)
- Metal detector indicator when equipped

**Pickup/Drop Mechanics** ✅ COMPLETE:
- Pickup via inspect UI (E key on selected item)
- Items drop on tile in front of player (based on last WASD direction)
- If blocked, drop on current tile

### 4. Anomaly System ✅ COMPLETE

**Visual Rendering:**
- **Running Mode**: All anomalies appear as identical semi-transparent purple overlays (z-index 2, above player)
- **Editor Mode**: Color-coded for easy placement (Purple=Gravitational, Gold=Philosopher's Stone, Orange=Rust)
- Anomaly overlays render above player sprite when player is on same tile

**Types:**

**Gravitational Anomaly** ✅:
- Pull: Player within 1 tile pulled in during world update
- Effect: Carry capacity reduced to 125
- Death: 5 turns inside anomaly = crushed
- Text: "You feel as if you weigh a thousand pounds. Every fiber in your body strains and creaks under the weight."

**Philosopher's Stone** ✅:
- Trigger: Player standing ON anomaly tile, ground items present
- Effect (valued items): Destroys 1 random item, replaces with equal/lesser value item, 5% chance → Fully Empty
- Effect (non-valued items): Shows mysterious flavor text, no transformation
- Dynamic item transformation using `ItemType::all_variants()` for maintainability
- Text: Atmospheric transformation messages ("The Scrap shimmers and becomes Glass Jar...")

**The Rust** ✅:
- Trigger: Player standing ON anomaly tile, metal items present (ground OR inventory)
- Effect: Destroys 1 random metal item → Rust Slag
- Text (ground): Clear descriptive "The [item] on the ground begins to rust rapidly..."
- Text (inventory): Vague sensory "The acrid smell of oxidation surrounds you..."

**Implementation:**
- Systems: `philosopher_stone_system`, `rust_anomaly_system` in `turn_processor.rs`
- Visual: `update_entity_colors_system` in `rendering.rs` (game state-aware)
- Dynamic item system with test coverage (`ItemType::all_variants()`)
- All anomaly effects generate atmospheric text in message log

### 5. Bolt Throwing System ✅ COMPLETE

**Controls:**
- `Q` key to enter ThrowingBolt phase (requires bolt in inventory)
- `WASD` to select direction
- Bolt fires automatically after direction selected
- `ESC` to cancel and return to PlayerTurn

**Mechanics:**
- Range: 5 tiles straight line (4 directions)
- Consumes 1 bolt from inventory
- Animation: 0.5 second flight, tile-by-tile with fading trail
- Collision: Stops at walls, anomalies, or max range
- Feedback: Message log describes what bolt hit

**Visual:**
- Red square indicator appears during direction selection
- Bolt sprite animates along path
- Trail sprites fade out after bolt passes
- All sprites cleaned up automatically

**Detection:**
- Reports anomaly type when bolt collides
- Reports wall/obstacle collisions
- Reports empty tiles at max range

**Technical Implementation:**
- Components: `BoltThrowingIndicator`, `BoltProjectile`, `BoltTrail`
- Turn phase: `TurnPhase::ThrowingBolt` (pauses game for direction input)
- Systems: `detect_bolt_throw_input_system`, `spawn_bolt_indicator_system`, `bolt_direction_input_system`, `animate_bolt_flight_system`, `update_bolt_trail_system`, `despawn_bolt_indicator_system`
- Files: `src/systems/bolt_throwing.rs`
- Fully integrated with inventory system

### 6. Ground Items ✅ COMPLETE
- **Visual**: Items.png sprite on tile (60% tile size, z=1)
- **Interaction**: Player stands on tile → press `E` → modal UI shows item list
- **Inspection**: Scrollable list with item name, weight, and value
- **Pickup**: Arrow keys to select, E to pickup selected item
- **Integration**: Full inventory system implemented

### 7. Metal Detector ✅ COMPLETE
- **Range**: 2 tiles Manhattan distance (dx + dy <= 2)
- **Feedback**: "⚠ METAL DETECTED" indicator in top-right corner
- **Activation**: Only active when Metal Detector in inventory
- **Detection**: Scans ground items for is_metal flag
- **No audio** (POC)

### 8. HUD Display ✅ COMPLETE
- Current weight / Max weight (actual inventory weight, red if overweight)
- Turn counter
- Message log (last 5 messages: anomaly effects, death, escape, pickup)
- Metal detector indicator (when equipped and metal detected)
- **Implementation**: See "Implementation Status" section above for full details

### 9. Win/Loss Conditions ✅ COMPLETE

**Contract System**:
- Mission Briefing screen on zone entry (EnteringZone phase)
- Shows active contracts (e.g., "Collect 3 artifacts of value 100 or greater")
- Press E to accept and begin

**Win Condition**:
- Reach exit tile → Extraction screen (ExitingZone phase)
- Shows contract completion status with [COMPLETE]/[FAILED] markers
- Press E to exit zone and restart with new contracts

**Loss Condition**:
- Death in Gravitational anomaly (timer reaches 0)
- Death screen appears (PlayerDead phase)
- Shows "Red has met his end in the Zone"
- Press E to restart with new stalker

**Implementation**:
- Contract validation checks inventory against contract requirements
- Auto-restart system: death/exit → transition to Editing → auto-restart to Running
- All game state resets: contracts, turn counter, message log
- Files: `src/systems/contract_ui.rs`, `src/resources/contract_system.rs`

## UI Standards

### Modal Screen Typography
All modal screens (Mission Briefing, Extraction, Death, Inspect, Inventory) use consistent font sizes:

| Element | Font Size | Usage |
|---------|-----------|-------|
| **Titles** | 24.0 | Main screen titles (e.g., "Mission Briefing", "DEATH", "Extraction Point") |
| **Subtitles/Headers** | 16.0 | Section headers (e.g., "Active Contracts:", "Contract Status:") |
| **Body Text** | 18.0 | Item descriptions, contract details, main content |
| **Help Text** | 16.0 | Input prompts (e.g., "E - Accept and Enter the Zone") |
| **Status Markers** | 16.0 | Completion indicators like [COMPLETE]/[FAILED] |

### Text Label Standards
- Use **text labels** instead of Unicode symbols for better font compatibility
- Status indicators: `[COMPLETE]` (green) and `[FAILED]` (red)
- Avoid Unicode characters like ✓ and ✗ which may not render correctly in default fonts

### Modal Styling
- Semi-transparent dark overlay (rgba 0,0,0,0.8) behind all modals
- Modal panels: dark gray background (rgb 0.15,0.15,0.15) with gray borders
- Minimum width: 500px, Maximum width: 700px
- Padding: 30-40px, Row gap: 15-20px
- Z-index: 100 for modal overlays

### Color Palette
- **Success/Positive**: Green (0.3, 0.9, 0.3) or (0.6, 0.9, 0.6)
- **Failure/Negative**: Red (0.9, 0.3, 0.3) or (0.9, 0.2, 0.2)
- **Highlight/Important**: Yellow (0.9, 0.9, 0.3)
- **Neutral Text**: White or light gray (0.8, 0.8, 0.8)

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
