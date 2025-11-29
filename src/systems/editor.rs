use bevy::prelude::*;
use crate::resources::{
    game_state::GameState,
    editor_state::{EditorState, EditorMode, EditorCursor},
    game_grid::{GameGrid, TileKind, EntityType, ItemType, Tile},
    map_data::MapData,
};
use crate::components::components::Position;
use crate::constants::TILE_SIZE;
use crate::systems::rendering::{grid_to_world, spawn_placed_entity};

const MAP_FILE_PATH: &str = "assets/maps/current.json";

// Marker component for cursor highlight sprite
#[derive(Component)]
pub struct CursorHighlight;

// Marker components for editor UI text
#[derive(Component)]
pub struct EditorHudRoot;

#[derive(Component)]
pub struct EditorModeText;

#[derive(Component)]
pub struct EditorSelectionText;

#[derive(Component)]
pub struct EditorCursorText;

// Toggle between Running and Editing states with F2
pub fn editor_toggle_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::F2) {
        match current_state.get() {
            GameState::Running => {
                next_state.set(GameState::Editing);
            }
            GameState::Editing => {
                next_state.set(GameState::Running);
            }
        }
    }
}

// Switch between Terrain, Entity, and Item placement modes with Tab
pub fn editor_mode_toggle_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut editor_state: ResMut<EditorState>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        editor_state.mode = match editor_state.mode {
            EditorMode::Terrain => EditorMode::Entity,
            EditorMode::Entity => EditorMode::Item,
            EditorMode::Item => EditorMode::Terrain,
        };
    }
}

// Select terrain, entity, or item type with number keys (mode-dependent)
pub fn editor_selection_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut editor_state: ResMut<EditorState>,
) {
    match editor_state.mode {
        EditorMode::Terrain => {
            if keyboard.just_pressed(KeyCode::Digit1) {
                editor_state.selected_terrain = TileKind::Floor;
            } else if keyboard.just_pressed(KeyCode::Digit2) {
                editor_state.selected_terrain = TileKind::Wall;
            }
        }
        EditorMode::Entity => {
            if keyboard.just_pressed(KeyCode::Digit1) {
                editor_state.selected_entity = EntityType::GravitationalAnomaly;
            } else if keyboard.just_pressed(KeyCode::Digit2) {
                editor_state.selected_entity = EntityType::PhilosopherStone;
            } else if keyboard.just_pressed(KeyCode::Digit3) {
                editor_state.selected_entity = EntityType::RustAnomaly;
            } else if keyboard.just_pressed(KeyCode::Digit4) {
                editor_state.selected_entity = EntityType::PlayerStart;
            } else if keyboard.just_pressed(KeyCode::Digit5) {
                editor_state.selected_entity = EntityType::Exit;
            } else if keyboard.just_pressed(KeyCode::Digit6) {
                editor_state.selected_entity = EntityType::LampPost;
            }
        }
        EditorMode::Item => {
            if keyboard.just_pressed(KeyCode::Digit1) {
                editor_state.selected_item = ItemType::FullyEmpty;
            } else if keyboard.just_pressed(KeyCode::Digit2) {
                editor_state.selected_item = ItemType::Scrap;
            } else if keyboard.just_pressed(KeyCode::Digit3) {
                editor_state.selected_item = ItemType::GlassJar;
            } else if keyboard.just_pressed(KeyCode::Digit4) {
                editor_state.selected_item = ItemType::Battery;
            }
        }
    }
}

// Track mouse position and convert to grid coordinates
pub fn editor_mouse_position_system(
    mut cursor: ResMut<EditorCursor>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    windows: Query<&Window>,
    grid: Res<GameGrid>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_screen_pos) = window.cursor_position() else {
        cursor.grid_position = None;
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        cursor.grid_position = None;
        return;
    };

    // Convert screen position to world position
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_screen_pos) else {
        cursor.grid_position = None;
        return;
    };

    // Convert world position to grid coordinates
    let grid_x_f = (world_pos.x / TILE_SIZE) + (grid.width as f32 / 2.0);
    let grid_y_f = (grid.height as f32 / 2.0) - (world_pos.y / TILE_SIZE);

    // Check if within bounds
    if grid_x_f >= 0.0 && grid_x_f < grid.width as f32 && grid_y_f >= 0.0 && grid_y_f < grid.height as f32 {
        cursor.grid_position = Some((grid_x_f as usize, grid_y_f as usize));
    } else {
        cursor.grid_position = None;
    }
}

// Update cursor highlight sprite position
pub fn editor_cursor_highlight_system(
    mut commands: Commands,
    cursor: Res<EditorCursor>,
    grid: Res<GameGrid>,
    mut highlight_query: Query<(Entity, &mut Transform, &mut Visibility), With<CursorHighlight>>,
) {
    if let Some((grid_x, grid_y)) = cursor.grid_position {
        let world_pos = grid_to_world(grid_x, grid_y, grid.width, grid.height);

        if let Ok((_, mut transform, mut visibility)) = highlight_query.single_mut() {
            // Update existing highlight
            transform.translation = Vec3::new(world_pos.x, world_pos.y, 5.0);
            *visibility = Visibility::Visible;
        } else {
            // Spawn new highlight
            commands.spawn((
                Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.3), // White semi-transparent
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(world_pos.x, world_pos.y, 5.0),
                CursorHighlight,
            ));
        }
    } else {
        // Hide highlight when cursor not over grid
        if let Ok((_, _, mut visibility)) = highlight_query.single_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

// Save/Load map with F3/F4 hotkeys
pub fn editor_save_load_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    grid: Res<GameGrid>,
    entity_query: Query<(Entity, &EntityType, &Position)>,
    ground_items_query: Query<(Entity, &crate::components::item::GroundItems, &Position), Without<EntityType>>,
    mut commands: Commands,
) {
    use crate::components::item::GroundItems;

    // F3: Save current map
    if keyboard.just_pressed(KeyCode::F3) {
        // Collect all placed entities
        let entities: Vec<(EntityType, usize, usize)> = entity_query
            .iter()
            .map(|(_, entity_type, pos)| (*entity_type, pos.x as usize, pos.y as usize))
            .collect();

        // Collect all ground items
        let ground_items: Vec<(GroundItems, usize, usize)> = ground_items_query
            .iter()
            .map(|(_, items, pos)| (items.clone(), pos.x as usize, pos.y as usize))
            .collect();

        let map_data = MapData::from_game_state(&grid, &entities, &ground_items);

        match map_data.save_to_file(MAP_FILE_PATH) {
            Ok(_) => info!("Map saved to {}", MAP_FILE_PATH),
            Err(e) => error!("Failed to save map: {}", e),
        }
    }

    // F4: Load map
    if keyboard.just_pressed(KeyCode::F4) {
        match MapData::load_from_file(MAP_FILE_PATH) {
            Ok(map_data) => {
                info!("Map loaded from {}", MAP_FILE_PATH);

                // Despawn all existing entities
                for (entity, _, _) in entity_query.iter() {
                    commands.entity(entity).despawn();
                }

                // Despawn all existing ground items
                for (entity, _, _) in ground_items_query.iter() {
                    commands.entity(entity).despawn();
                }

                // Replace the grid (this will trigger tile sprite reload)
                let new_grid = map_data.to_game_grid();
                let grid_width = new_grid.width;
                let grid_height = new_grid.height;
                commands.insert_resource(new_grid);

                // Spawn entities from loaded map
                for placed_entity in &map_data.entities {
                    spawn_placed_entity(
                        &mut commands,
                        placed_entity.entity_type.into(),
                        placed_entity.x,
                        placed_entity.y,
                        grid_width,
                        grid_height,
                    );
                }

                // Spawn ground items from loaded map
                for placed_items in &map_data.items {
                    let mut ground_items = GroundItems::new();
                    for item in &placed_items.items {
                        ground_items.add_item(item.clone());
                    }

                    commands.spawn((
                        Position {
                            x: placed_items.x as i32,
                            y: placed_items.y as i32,
                        },
                        ground_items,
                    ));
                }

                info!("Loaded {}x{} map with {} entities and {} item locations",
                    map_data.width, map_data.height, map_data.entities.len(), map_data.items.len());
            }
            Err(e) => error!("Failed to load map: {}", e),
        }
    }
}

// Place terrain, entities, or items with mouse clicks
pub fn editor_placement_system(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor: Res<EditorCursor>,
    editor_state: Res<EditorState>,
    mut grid: ResMut<GameGrid>,
    entity_query: Query<(Entity, &Position, &EntityType)>,
    mut ground_items_query: Query<(Entity, &Position, &mut crate::components::item::GroundItems), Without<EntityType>>,
) {
    use crate::components::item::{Item, GroundItems};

    let Some((grid_x, grid_y)) = cursor.grid_position else {
        return;
    };

    // Left-click: Place terrain, entity, or item
    if mouse.just_pressed(MouseButton::Left) {
        match editor_state.mode {
            EditorMode::Terrain => {
                // Update the terrain tile
                let new_tile = Tile::new(editor_state.selected_terrain);
                grid.set_tile(grid_x, grid_y, new_tile);
            }
            EditorMode::Entity => {
                // Check if entity already exists at this position
                let existing_entity = entity_query.iter().find(|(_, pos, _)| {
                    pos.x == grid_x as i32 && pos.y == grid_y as i32
                });

                // Don't place if there's already an entity here
                if existing_entity.is_none() {
                    spawn_placed_entity(
                        &mut commands,
                        editor_state.selected_entity,
                        grid_x,
                        grid_y,
                        grid.width,
                        grid.height,
                    );
                }
            }
            EditorMode::Item => {
                // Check if there's already a GroundItems entity at this position
                let existing_ground_items = ground_items_query.iter_mut().find(|(_, pos, _)| {
                    pos.x == grid_x as i32 && pos.y == grid_y as i32
                });

                if let Some((_, _, mut ground_items)) = existing_ground_items {
                    // Add item to existing GroundItems
                    let item: Item = editor_state.selected_item.into();
                    ground_items.add_item(item);
                } else {
                    // Create new GroundItems entity
                    let item: Item = editor_state.selected_item.into();
                    let mut ground_items = GroundItems::new();
                    ground_items.add_item(item);

                    commands.spawn((
                        Position {
                            x: grid_x as i32,
                            y: grid_y as i32,
                        },
                        ground_items,
                    ));
                }
            }
        }
    }

    // Right-click: Delete entity, reset tile to Floor, or remove all items
    if mouse.just_pressed(MouseButton::Right) {
        match editor_state.mode {
            EditorMode::Terrain => {
                // Reset to floor
                let floor_tile = Tile::new(TileKind::Floor);
                grid.set_tile(grid_x, grid_y, floor_tile);
            }
            EditorMode::Entity => {
                // Find and delete entity at cursor position
                for (entity, pos, _) in entity_query.iter() {
                    if pos.x == grid_x as i32 && pos.y == grid_y as i32 {
                        commands.entity(entity).despawn();
                        break; // Only delete one entity
                    }
                }
            }
            EditorMode::Item => {
                // Find and delete GroundItems entity at cursor position
                for (entity, pos, _) in ground_items_query.iter() {
                    if pos.x == grid_x as i32 && pos.y == grid_y as i32 {
                        commands.entity(entity).despawn();
                        break;
                    }
                }
            }
        }
    }
}

// Spawn editor HUD when entering editor mode
pub fn spawn_editor_hud_system(
    mut commands: Commands,
    hud_query: Query<Entity, With<EditorHudRoot>>,
) {
    // Only spawn if doesn't exist
    if hud_query.iter().next().is_some() {
        return;
    }

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },
            EditorHudRoot,
        ))
        .with_children(|parent| {
            // Mode line
            parent.spawn((
                Text::new("EDITOR MODE | F2: Play | Tab: Switch Mode"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                EditorModeText,
            ));

            // Selection line
            parent.spawn((
                Text::new("TERRAIN: Floor"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                EditorSelectionText,
            ));

            // Cursor position line
            parent.spawn((
                Text::new("Cursor: --"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                EditorCursorText,
            ));
        });
}

// Update editor HUD text
pub fn update_editor_hud_system(
    editor_state: Res<EditorState>,
    cursor: Res<EditorCursor>,
    mut selection_text_query: Query<&mut Text, (With<EditorSelectionText>, Without<EditorCursorText>)>,
    mut cursor_text_query: Query<&mut Text, (With<EditorCursorText>, Without<EditorSelectionText>)>,
) {
    use crate::resources::game_grid::ItemType;

    // Update selection text
    if let Ok(mut text) = selection_text_query.single_mut() {
        let selection_str = match editor_state.mode {
            EditorMode::Terrain => {
                match editor_state.selected_terrain {
                    TileKind::Floor => "TERRAIN: 1=Floor, 2=Wall | Selected: Floor (1)".to_string(),
                    TileKind::Wall => "TERRAIN: 1=Floor, 2=Wall | Selected: Wall (2)".to_string(),
                }
            }
            EditorMode::Entity => {
                let selected = match editor_state.selected_entity {
                    EntityType::GravitationalAnomaly => "Gravitational Anomaly (1)",
                    EntityType::PhilosopherStone => "Philosopher's Stone (2)",
                    EntityType::RustAnomaly => "Rust Anomaly (3)",
                    EntityType::PlayerStart => "Player Start (4)",
                    EntityType::Exit => "Exit (5)",
                    EntityType::LampPost => "Lamp Post (6)",
                };
                format!("ENTITY: 1=Grav, 2=Phil, 3=Rust, 4=Start, 5=Exit, 6=Lamp | Selected: {}", selected)
            }
            EditorMode::Item => {
                let selected = match editor_state.selected_item {
                    ItemType::FullyEmpty => "Fully Empty (1)",
                    ItemType::Scrap => "Scrap (2)",
                    ItemType::GlassJar => "Glass Jar (3)",
                    ItemType::Battery => "Battery (4)",
                };
                format!("ITEM: 1=FullyEmpty, 2=Scrap, 3=Glass, 4=Battery | Selected: {}", selected)
            }
        };
        **text = selection_str;
    }

    // Update cursor position text
    if let Ok(mut text) = cursor_text_query.single_mut() {
        if let Some((x, y)) = cursor.grid_position {
            **text = format!("Cursor: ({}, {})", x, y);
        } else {
            **text = "Cursor: --".to_string();
        }
    }
}

// Show/hide editor HUD based on game state
pub fn toggle_editor_hud_visibility_system(
    current_state: Res<State<GameState>>,
    mut hud_query: Query<&mut Visibility, With<EditorHudRoot>>,
) {
    if let Ok(mut visibility) = hud_query.single_mut() {
        *visibility = match current_state.get() {
            GameState::Editing => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
