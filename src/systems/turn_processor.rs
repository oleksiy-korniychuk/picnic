use bevy::prelude::*;
use rand::prelude::*;
use crate::components::components::{Player, Position, GravitationalAnomalyTimer};
use crate::components::{inventory::Inventory, item::{Item, GroundItems}};
use crate::resources::{
    game_grid::{EntityType, ItemType},
    game_state::GameState,
    turn_state::{TurnPhase, TurnCounter},
    message_log::MessageLog,
};

/// Checks if player is adjacent to a Gravitational Anomaly and pulls them in
/// This is the first system in the WorldUpdate phase chain
/// NOTE: Only pulls players who don't have a timer (fresh captures, not escaping players)
pub fn gravitational_pull_system(
    mut player_query: Query<(Entity, &mut Position, Option<&GravitationalAnomalyTimer>), With<Player>>,
    anomaly_query: Query<(&Position, &EntityType), Without<Player>>,
    mut commands: Commands,
    mut message_log: ResMut<MessageLog>,
) {
    let Ok((player_entity, mut player_pos, timer_opt)) = player_query.single_mut() else {
        return;
    };

    // Don't pull if player already has a timer (they're trying to escape)
    // This allows them to move away from the anomaly
    if timer_opt.is_some() {
        return;
    }

    // Find all gravitational anomalies
    for (anomaly_pos, entity_type) in anomaly_query.iter() {
        // Only process Gravitational Anomalies
        if !matches!(entity_type, EntityType::GravitationalAnomaly) {
            continue;
        }

        // Check 4-directional adjacency (distance of 1)
        let dx = (player_pos.x - anomaly_pos.x).abs();
        let dy = (player_pos.y - anomaly_pos.y).abs();

        // Adjacent means exactly 1 tile away in one direction, 0 in the other
        if (dx == 1 && dy == 0) || (dx == 0 && dy == 1) {
            // Pull player toward anomaly (move 1 tile closer)
            if player_pos.x < anomaly_pos.x {
                player_pos.x += 1;
            } else if player_pos.x > anomaly_pos.x {
                player_pos.x -= 1;
            } else if player_pos.y < anomaly_pos.y {
                player_pos.y += 1;
            } else if player_pos.y > anomaly_pos.y {
                player_pos.y -= 1;
            }

            message_log.add_message("Gravitational anomaly pulls you in!");
            info!("Gravitational anomaly pulled player to ({}, {})", player_pos.x, player_pos.y);

            // Check if player is now on the anomaly
            if player_pos.x == anomaly_pos.x && player_pos.y == anomaly_pos.y {
                // Add timer component - player has entered the anomaly
                commands.entity(player_entity).insert(GravitationalAnomalyTimer(5));
                message_log.add_message("Immense pressure... 5 turns to escape!");
                warn!("Player entered gravitational anomaly! 5 turns to escape or die!");
            }

            // Only process one anomaly pull per turn
            break;
        }
    }
}

/// Philosopher's Stone anomaly effect
/// Triggers when player is standing ON the anomaly tile
/// Transforms ground items with value into equal/lesser value items (5% chance for Fully Empty)
/// Shows mysterious flavor text for non-valued items (no transformation)
pub fn philosopher_stone_system(
    mut commands: Commands,
    player_query: Query<&Position, With<Player>>,
    anomaly_query: Query<(&Position, &EntityType), Without<Player>>,
    mut ground_items_query: Query<(Entity, &Position, &mut GroundItems)>,
    mut message_log: ResMut<MessageLog>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    // Find Philosopher's Stone at player's position
    let philosopher_stone_here = anomaly_query.iter().any(|(anomaly_pos, entity_type)| {
        matches!(entity_type, EntityType::PhilosopherStone)
            && anomaly_pos.x == player_pos.x
            && anomaly_pos.y == player_pos.y
    });

    if !philosopher_stone_here {
        return;
    }

    // Find ground items at player's position
    for (entity, pos, mut ground_items) in ground_items_query.iter_mut() {
        if pos.x != player_pos.x || pos.y != player_pos.y {
            continue;
        }

        if ground_items.items.is_empty() {
            continue;
        }

        // Collect indices of valued items
        let valued_indices: Vec<usize> = ground_items
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.value.is_some())
            .map(|(idx, _)| idx)
            .collect();

        if valued_indices.is_empty() {
            // Only non-valued items present - show mysterious flavor text
            let messages = [
                "The anomaly pulses with strange energy, but the items remain unchanged.",
                "Reality shifts around you, but nothing happens.",
                "Strange forces swirl, then dissipate.",
                "You sense the anomaly trying to reshape what lies before you, but it cannot.",
            ];
            let msg = messages.choose(&mut rand::rng()).unwrap();
            message_log.add_message(*msg);
            info!("Philosopher's Stone: Non-valued items, no transformation");
            return;
        }

        // Select random valued item index
        let selected_idx = *valued_indices.choose(&mut rand::rng()).unwrap();
        let original_item = &ground_items.items[selected_idx];
        let original_value = original_item.value.unwrap();
        let original_name = original_item.name.clone();

        // Generate transformation
        let mut rng = rand::rng();
        let new_item = if rng.random_bool(0.05) {
            // 5% chance for Fully Empty
            Item::from(ItemType::FullyEmpty)
        } else {
            // Get items with value <= original value
            let eligible: Vec<ItemType> = ItemType::all_variants()
                .into_iter()
                .filter(|item_type| {
                    let item: Item = (*item_type).into();
                    item.value.map_or(false, |v| v <= original_value)
                })
                .collect();

            let selected_type = eligible.choose(&mut rng).unwrap();
            Item::from(*selected_type)
        };

        // Remove old item and add new one
        ground_items.items.remove(selected_idx);
        ground_items.add_item(new_item.clone());

        // Generate atmospheric message
        let message = if new_item.name == "Fully Empty" {
            format!(
                "The fabric of reality tears. A Fully Empty materializes where {} once was.",
                original_name
            )
        } else {
            let transformations = [
                format!("The {} shimmers with impossible light and becomes {}.", original_name, new_item.name),
                format!("Reality fractures. The {} transforms into {}.", original_name, new_item.name),
                format!("The anomaly pulses. Where {} lay, now rests {}.", original_name, new_item.name),
            ];
            transformations.choose(&mut rng).unwrap().clone()
        };

        message_log.add_message(&message);
        info!("Philosopher's Stone: Transformed {} → {}", original_name, new_item.name);

        // If ground items is now empty, despawn the entity
        if ground_items.is_empty() {
            commands.entity(entity).despawn();
        }

        return; // Only transform one item per turn
    }
}

/// The Rust anomaly effect
/// Triggers when player is standing ON the anomaly tile
/// Rusts metal items from ground OR player inventory
/// Ground items: clear descriptive message
/// Inventory items: vague sensory message (player doesn't know what rusted until they check)
pub fn rust_anomaly_system(
    mut commands: Commands,
    mut player_query: Query<(&Position, &mut Inventory), With<Player>>,
    anomaly_query: Query<(&Position, &EntityType)>,
    mut ground_items_query: Query<(Entity, &Position, &mut GroundItems)>,
    mut message_log: ResMut<MessageLog>,
) {
    let Ok((player_pos, mut player_inventory)) = player_query.single_mut() else {
        return;
    };

    // Find Rust anomaly at player's position
    let rust_here = anomaly_query.iter().any(|(anomaly_pos, entity_type)| {
        matches!(entity_type, EntityType::RustAnomaly)
            && anomaly_pos.x == player_pos.x
            && anomaly_pos.y == player_pos.y
    });

    if !rust_here {
        return;
    }

    // Collect all metal items from ground and inventory
    enum MetalSource {
        Ground(Entity, usize), // entity, item_index
        Inventory(usize),      // item_index
    }

    let mut metal_items: Vec<(MetalSource, String)> = Vec::new();

    // Check ground items
    for (entity, pos, ground_items) in ground_items_query.iter() {
        if pos.x == player_pos.x && pos.y == player_pos.y {
            for (idx, item) in ground_items.items.iter().enumerate() {
                if item.is_metal {
                    metal_items.push((MetalSource::Ground(entity, idx), item.name.clone()));
                }
            }
        }
    }

    // Check inventory items
    for (idx, item) in player_inventory.items.iter().enumerate() {
        if item.is_metal {
            metal_items.push((MetalSource::Inventory(idx), item.name.clone()));
        }
    }

    if metal_items.is_empty() {
        return; // No metal items to rust
    }

    // Select random metal item
    let (source, item_name) = metal_items.choose(&mut rand::rng()).unwrap();

    match source {
        MetalSource::Ground(entity, item_idx) => {
            // Rust ground item - clear descriptive message
            if let Ok((ent, _, mut ground_items)) = ground_items_query.get_mut(*entity) {
                ground_items.items.remove(*item_idx);
                ground_items.add_item(Item::from(ItemType::RustSlag));

                let message = format!(
                    "The {} on the ground begins to rust rapidly before your very eyes. In an instant, it melts into a rusty glob.",
                    item_name
                );
                message_log.add_message(&message);
                info!("Rust anomaly: Rusted ground item {} → Rust Slag", item_name);

                // Despawn if no items remain
                if ground_items.is_empty() {
                    commands.entity(ent).despawn();
                }
            }
        }
        MetalSource::Inventory(item_idx) => {
            // Rust inventory item - vague sensory message
            player_inventory.items.remove(*item_idx);
            player_inventory.add_item(Item::from(ItemType::RustSlag));

            let messages = [
                "The acrid smell of oxidation surrounds you.",
                "You sense something shifting in your pack.",
                "A metallic tang fills the air.",
                "The scent of rust and iron overwhelms you.",
                "Something heavy settles differently at your side.",
            ];
            let msg = messages.choose(&mut rand::rng()).unwrap();
            message_log.add_message(*msg);
            info!("Rust anomaly: Rusted inventory item {} → Rust Slag", item_name);
        }
    }
}

/// Updates the gravitational anomaly timer
/// Decrements if player is within range (on or adjacent), removes if player escaped to safe distance
pub fn gravitational_timer_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Position, Option<&mut GravitationalAnomalyTimer>), With<Player>>,
    anomaly_query: Query<(&Position, &EntityType), Without<Player>>,
    mut message_log: ResMut<MessageLog>,
) {
    let Ok((player_entity, player_pos, timer_opt)) = player_query.single_mut() else {
        return;
    };

    // Check if player has a timer
    let Some(mut timer) = timer_opt else {
        return;
    };

    // Check if player is within range (≤1 tile) of ANY gravitational anomaly
    // Within range means: on the anomaly OR adjacent to it
    let within_range = anomaly_query.iter().any(|(anomaly_pos, entity_type)| {
        if !matches!(entity_type, EntityType::GravitationalAnomaly) {
            return false;
        }

        let dx = (player_pos.x - anomaly_pos.x).abs();
        let dy = (player_pos.y - anomaly_pos.y).abs();

        // Within range: on anomaly (dx==0 && dy==0) OR adjacent (dx+dy==1)
        // This covers: same tile, or exactly 1 tile away in 4 directions
        (dx == 0 && dy == 0) || (dx == 1 && dy == 0) || (dx == 0 && dy == 1)
    });

    if within_range {
        // Still in danger - decrement timer
        timer.0 = timer.0.saturating_sub(1);
        message_log.add_message(format!("Crushing pressure! {} turns left!", timer.0));
        warn!("Gravitational anomaly! {} turns remaining!", timer.0);
    } else {
        // Player escaped to safe distance (>1 tile from all anomalies) - remove timer
        commands.entity(player_entity).remove::<GravitationalAnomalyTimer>();
        message_log.add_message("You break free from the anomaly!");
        info!("Player escaped gravitational anomaly!");
    }
}

/// Checks if player has died and handles death
/// Currently only checks gravitational anomaly death (timer reaches 0)
pub fn death_check_system(
    player_query: Query<&GravitationalAnomalyTimer, With<Player>>,
    mut next_phase: ResMut<NextState<TurnPhase>>,
    mut message_log: ResMut<MessageLog>,
) {
    let Ok(timer) = player_query.single() else {
        return;
    };

    if timer.0 == 0 {
        // Player died - transition to death screen
        message_log.add_message("You are crushed to death!");
        next_phase.set(TurnPhase::PlayerDead);
        error!("DEATH: Player was crushed by gravitational anomaly!");
    }
}

/// Increments the turn counter
pub fn increment_turn_counter_system(
    mut turn_counter: ResMut<TurnCounter>,
) {
    turn_counter.0 += 1;
    info!("Turn {}", turn_counter.0);
}

/// Transitions back to PlayerTurn phase
/// This is the last system in the WorldUpdate chain
pub fn transition_to_player_turn_system(
    mut next_phase: ResMut<NextState<TurnPhase>>,
) {
    next_phase.set(TurnPhase::PlayerTurn);
}
