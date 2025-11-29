use bevy::prelude::*;
use crate::components::{
    components::{Player, Position},
    inventory::Inventory,
    item::GroundItems,
};

/// Marker component for the metal detector indicator in HUD
#[derive(Component)]
pub struct MetalDetectorIndicator;

/// Spawns the metal detector indicator when entering Running mode
pub fn spawn_metal_detector_indicator_system(
    mut commands: Commands,
    existing: Query<Entity, With<MetalDetectorIndicator>>,
) {
    // Don't spawn if already exists
    if existing.iter().next().is_some() {
        return;
    }

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            top: Val::Px(20.0),
            width: Val::Px(200.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
        BorderColor(Color::srgb(0.5, 0.5, 0.5)),
        Visibility::Hidden, // Hidden by default
        MetalDetectorIndicator,
        ZIndex(50),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("⚠ METAL DETECTED"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.8, 0.0)), // Yellow/gold
        ));
    });
}

/// Despawns the metal detector indicator when exiting Running mode
pub fn despawn_metal_detector_indicator_system(
    mut commands: Commands,
    query: Query<Entity, With<MetalDetectorIndicator>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Updates metal detector indicator visibility based on nearby metal items
pub fn update_metal_detector_system(
    player_query: Query<(&Position, &Inventory), With<Player>>,
    ground_items_query: Query<(&Position, &GroundItems)>,
    mut indicator_query: Query<&mut Visibility, With<MetalDetectorIndicator>>,
) {
    let Ok(mut visibility) = indicator_query.single_mut() else {
        return;
    };

    let Ok((player_pos, inventory)) = player_query.single() else {
        *visibility = Visibility::Hidden;
        return;
    };

    // Check if player has metal detector
    if !inventory.has_metal_detector() {
        *visibility = Visibility::Hidden;
        return;
    }

    // Scan for metal items within 2-tile radius
    let mut metal_detected = false;
    for (item_pos, ground_items) in ground_items_query.iter() {
        // Calculate Manhattan distance
        let dx = (player_pos.x - item_pos.x).abs();
        let dy = (player_pos.y - item_pos.y).abs();
        let distance = dx + dy;

        // Check if within 2-tile radius
        if distance <= 2 {
            // Check if any items are metal
            if ground_items.items.iter().any(|item| item.is_metal) {
                metal_detected = true;
                break;
            }
        }
    }

    *visibility = if metal_detected {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}
