use bevy::prelude::*;
use crate::components::{
    components::{Player, Position},
    inventory::Inventory,
    item::GroundItems,
};
use crate::resources::{
    turn_state::TurnPhase,
    message_log::MessageLog,
    game_grid::{GameGrid, TileKind, EntityType, ItemType},
};
use crate::systems::rendering::grid_to_world;
use crate::constants::TILE_SIZE;

// --- Components ---

/// Marker component for the bolt throwing mode indicator (red square)
#[derive(Component)]
pub struct BoltThrowingIndicator;

/// Component for a bolt projectile in flight
#[derive(Component)]
pub struct BoltProjectile {
    pub direction: (i32, i32),  // Movement delta (dx, dy)
    pub tiles_traveled: u32,
    pub max_range: u32,
    pub animation_timer: Timer,
}

/// Component for bolt trail sprites that fade out
#[derive(Component)]
pub struct BoltTrail {
    pub fade_timer: Timer,
}

// --- System 1: Detect Q key to enter ThrowingBolt mode ---

/// Detects Q key press and transitions to ThrowingBolt phase if player has bolts
pub fn detect_bolt_throw_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Inventory, With<Player>>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
    mut message_log: ResMut<MessageLog>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        if let Ok(inventory) = player_query.single() {
            // Check if player has at least one bolt
            let has_bolt = inventory.items.iter().any(|item| item.name == "Bolt");

            if has_bolt {
                next_phase.set(TurnPhase::ThrowingBolt);
                info!("Entering bolt throwing mode");
            } else {
                message_log.add_message("You don't have any bolts to throw!");
            }
        }
    }
}

// --- System 2: Spawn visual indicator (red square) ---

/// Spawns a small red square in top-right corner of player tile when entering ThrowingBolt phase
pub fn spawn_bolt_indicator_system(
    mut commands: Commands,
    player_query: Query<&Position, With<Player>>,
    grid: Res<GameGrid>,
) {
    if let Ok(player_pos) = player_query.single() {
        // Convert player grid position to world coordinates
        let world_pos = grid_to_world(
            player_pos.x as usize,
            player_pos.y as usize,
            grid.width,
            grid.height,
        );

        // Position indicator in top-right corner of player tile
        // Offset by quarter tile size to the right and up
        let indicator_x = world_pos.x + (TILE_SIZE * 0.25);
        let indicator_y = world_pos.y - (TILE_SIZE * 0.25);

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0), // Red
                custom_size: Some(Vec2::new(TILE_SIZE * 0.2, TILE_SIZE * 0.2)), // Small square
                ..default()
            },
            Transform::from_xyz(indicator_x, indicator_y, 15.0), // Above player
            BoltThrowingIndicator,
        ));

        info!("Spawned bolt throwing indicator at player position");
    }
}

/// Despawns the bolt throwing indicator when exiting ThrowingBolt phase
pub fn despawn_bolt_indicator_system(
    mut commands: Commands,
    indicator_query: Query<Entity, With<BoltThrowingIndicator>>,
) {
    for entity in indicator_query.iter() {
        commands.entity(entity).despawn();
        info!("Despawned bolt throwing indicator");
    }
}

// --- System 3: Handle direction input (WASD) or cancel (Q) ---

/// Handles WASD for direction selection or Q to cancel throwing mode
pub fn bolt_direction_input_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Position, &mut Inventory), With<Player>>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
    mut message_log: ResMut<MessageLog>,
    asset_server: Res<AssetServer>,
    grid: Res<GameGrid>,
) {
    // Check for cancel input (Q)
    if keyboard.just_pressed(KeyCode::KeyQ) {
        message_log.add_message("You put away the bolt.");
        next_phase.set(TurnPhase::PlayerTurn);
        return;
    }

    // Check for direction input (WASD)
    let direction = if keyboard.just_pressed(KeyCode::KeyW) {
        Some((0, -1)) // Up
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        Some((0, 1)) // Down
    } else if keyboard.just_pressed(KeyCode::KeyA) {
        Some((-1, 0)) // Left
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        Some((1, 0)) // Right
    } else {
        None
    };

    if let Some((dx, dy)) = direction {
        if let Ok((player_pos, mut inventory)) = player_query.single_mut() {
            // Remove one bolt from inventory
            if let Some(bolt_index) = inventory.items.iter().position(|item| item.name == "Bolt") {
                inventory.items.remove(bolt_index);
                info!("Removed bolt from inventory, {} bolts remaining",
                      inventory.items.iter().filter(|i| i.name == "Bolt").count());

                // Spawn bolt projectile
                let world_pos = grid_to_world(
                    player_pos.x as usize,
                    player_pos.y as usize,
                    grid.width,
                    grid.height,
                );

                let texture = asset_server.load("Red.png"); // Reuse Red.png for bolt visual

                commands.spawn((
                    Sprite {
                        image: texture,
                        color: Color::srgb(0.8, 0.8, 0.0), // Yellow tint for bolt
                        custom_size: Some(Vec2::new(TILE_SIZE * 0.3, TILE_SIZE * 0.3)),
                        ..default()
                    },
                    Transform::from_xyz(world_pos.x, world_pos.y, 12.0),
                    BoltProjectile {
                        direction: (dx, dy),
                        tiles_traveled: 0,
                        max_range: 5,
                        animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating), // 0.1s per tile = 0.5s total
                    },
                    *player_pos, // Start at player position
                ));

                info!("Spawned bolt projectile heading {:?}", (dx, dy));
            }
        }
    }
}

// --- System 4: Animate bolt flight and handle collisions ---

/// Animates bolt projectiles, handles collisions, and places bolt on ground when stopped
pub fn animate_bolt_flight_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut BoltProjectile, &mut Position, &mut Transform), (With<BoltProjectile>, Without<EntityType>, Without<GroundItems>)>,
    grid: Res<GameGrid>,
    entity_query: Query<(&Position, &EntityType), (With<EntityType>, Without<BoltProjectile>, Without<GroundItems>)>,
    mut ground_items_query: Query<(&Position, &mut GroundItems), (With<GroundItems>, Without<BoltProjectile>, Without<EntityType>)>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
    mut message_log: ResMut<MessageLog>,
    asset_server: Res<AssetServer>,
) {
    for (projectile_entity, mut projectile, mut pos, mut transform) in projectile_query.iter_mut() {
        projectile.animation_timer.tick(time.delta());

        if projectile.animation_timer.just_finished() {
            // Calculate next position
            let next_x = pos.x + projectile.direction.0;
            let next_y = pos.y + projectile.direction.1;

            // Check if out of bounds
            if next_x < 0 || next_y < 0 || next_x >= grid.width as i32 || next_y >= grid.height as i32 {
                // Out of bounds - stop here
                message_log.add_message("The bolt flies out of sight.");
                finalize_bolt(
                    &mut commands,
                    projectile_entity,
                    *pos,
                    &mut ground_items_query,
                    &mut next_phase,
                );
                continue;
            }

            // Check for wall collision
            if let Some(tile) = grid.get_tile(next_x as usize, next_y as usize) {
                if tile.kind == TileKind::Wall {
                    message_log.add_message("The bolt clangs against the wall.");
                    finalize_bolt(
                        &mut commands,
                        projectile_entity,
                        *pos,
                        &mut ground_items_query,
                        &mut next_phase,
                    );
                    continue;
                }
            }

            // Check for anomaly collision at next position
            let next_pos = Position { x: next_x, y: next_y };
            if let Some(anomaly_detected) = check_anomaly_collision(&entity_query, &next_pos) {
                // Bolt hits anomaly - generate appropriate message
                let detection_message = match anomaly_detected {
                    EntityType::GravitationalAnomaly => {
                        "The bolt curves sharply and falls to the ground near a gravitational distortion."
                    },
                    EntityType::PhilosopherStone => {
                        "The bolt strikes something shimmering and falls to the ground."
                    },
                    EntityType::RustAnomaly => {
                        "The bolt strikes something and begins to oxidize rapidly."
                    },
                    _ => "The bolt strikes something unusual.",
                };
                message_log.add_message(detection_message);

                // Stop at this position
                *pos = next_pos;
                finalize_bolt(
                    &mut commands,
                    projectile_entity,
                    *pos,
                    &mut ground_items_query,
                    &mut next_phase,
                );
                continue;
            }

            // Move to next tile
            projectile.tiles_traveled += 1;
            *pos = next_pos;

            // Update visual position
            let world_pos = grid_to_world(
                pos.x as usize,
                pos.y as usize,
                grid.width,
                grid.height,
            );
            transform.translation.x = world_pos.x;
            transform.translation.y = world_pos.y;

            // Spawn trail sprite at previous position
            spawn_trail(&mut commands, transform.translation.x, transform.translation.y, &asset_server);

            // Check if reached max range
            if projectile.tiles_traveled >= projectile.max_range {
                message_log.add_message("The bolt falls to the ground harmlessly.");
                finalize_bolt(
                    &mut commands,
                    projectile_entity,
                    *pos,
                    &mut ground_items_query,
                    &mut next_phase,
                );
            }
        }
    }
}

/// Checks if there's an anomaly at the given position
fn check_anomaly_collision(
    entity_query: &Query<(&Position, &EntityType), (With<EntityType>, Without<BoltProjectile>, Without<GroundItems>)>,
    pos: &Position,
) -> Option<EntityType> {
    for (entity_pos, entity_type) in entity_query.iter() {
        if entity_pos.x == pos.x && entity_pos.y == pos.y {
            match entity_type {
                EntityType::GravitationalAnomaly
                | EntityType::PhilosopherStone
                | EntityType::RustAnomaly => {
                    return Some(*entity_type);
                },
                _ => {}
            }
        }
    }
    None
}

/// Finalizes bolt flight: adds bolt to ground, despawns projectile, transitions to WorldUpdate
fn finalize_bolt(
    commands: &mut Commands,
    projectile_entity: Entity,
    final_pos: Position,
    ground_items_query: &mut Query<(&Position, &mut GroundItems), (With<GroundItems>, Without<BoltProjectile>, Without<EntityType>)>,
    next_phase: &mut ResMut<NextState<TurnPhase>>,
) {
    // Add bolt to ground at final position
    add_bolt_to_ground(commands, final_pos, ground_items_query);

    // Despawn projectile
    commands.entity(projectile_entity).despawn();

    // Transition to WorldUpdate phase (consumes turn)
    next_phase.set(TurnPhase::WorldUpdate);

    info!("Bolt finalized at position ({}, {})", final_pos.x, final_pos.y);
}

/// Adds a bolt item to the ground at the specified position
fn add_bolt_to_ground(
    commands: &mut Commands,
    pos: Position,
    ground_items_query: &mut Query<(&Position, &mut GroundItems), (With<GroundItems>, Without<BoltProjectile>, Without<EntityType>)>,
) {
    // Find existing GroundItems entity at this position
    let mut found = false;
    for (ground_pos, mut ground_items) in ground_items_query.iter_mut() {
        if ground_pos.x == pos.x && ground_pos.y == pos.y {
            ground_items.add_item(ItemType::Bolt.into());
            found = true;
            info!("Added bolt to existing GroundItems at ({}, {})", pos.x, pos.y);
            break;
        }
    }

    // If no GroundItems entity exists at this position, create one
    if !found {
        let mut new_ground_items = GroundItems::new();
        new_ground_items.add_item(ItemType::Bolt.into());

        commands.spawn((
            pos,
            new_ground_items,
        ));

        info!("Created new GroundItems with bolt at ({}, {})", pos.x, pos.y);
    }
}

/// Spawns a trail sprite at the given position
fn spawn_trail(commands: &mut Commands, x: f32, y: f32, asset_server: &Res<AssetServer>) {
    let texture = asset_server.load("Red.png");

    commands.spawn((
        Sprite {
            image: texture,
            color: Color::srgba(0.8, 0.8, 0.0, 0.5), // Semi-transparent yellow
            custom_size: Some(Vec2::new(TILE_SIZE * 0.2, TILE_SIZE * 0.2)),
            ..default()
        },
        Transform::from_xyz(x, y, 11.0),
        BoltTrail {
            fade_timer: Timer::from_seconds(0.3, TimerMode::Once),
        },
    ));
}

// --- System 5: Update trail fade ---

/// Fades out and despawns trail sprites
pub fn update_bolt_trail_system(
    mut commands: Commands,
    time: Res<Time>,
    mut trail_query: Query<(Entity, &mut BoltTrail, &mut Sprite)>,
) {
    for (entity, mut trail, mut sprite) in trail_query.iter_mut() {
        trail.fade_timer.tick(time.delta());

        // Update alpha based on remaining time
        let alpha = trail.fade_timer.fraction_remaining();
        sprite.color = Color::srgba(0.8, 0.8, 0.0, alpha * 0.5);

        // Despawn when fade complete
        if trail.fade_timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
