use bevy::prelude::*;
use crate::components::components::{Player, Position, GravitationalAnomalyTimer};
use crate::resources::{
    game_grid::EntityType,
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

/// Placeholder for other anomaly effects (Philosopher's Stone, Rust)
/// Will be implemented in later POC phases
pub fn anomaly_effects_system() {
    // TODO: Implement Philosopher's Stone and Rust anomaly effects
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
    mut commands: Commands,
    player_query: Query<(Entity, &GravitationalAnomalyTimer), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut message_log: ResMut<MessageLog>,
) {
    let Ok((player_entity, timer)) = player_query.single() else {
        return;
    };

    if timer.0 == 0 {
        // Player died - despawn and return to editing
        message_log.add_message("You are crushed to death!");
        commands.entity(player_entity).despawn();
        next_state.set(GameState::Editing);
        error!("DEATH: Player was crushed by gravitational anomaly!");
        // TODO: Full reset in later POC phase
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
