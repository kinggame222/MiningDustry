use bevy::prelude::*;
use crate::particles::base::{PIXEL_SIZE, Particle, ParticleType};
use rand::prelude::*;

// Structure pour les effets temporaires
#[derive(Component)]
pub struct Effect {
    pub lifetime: f32,
    pub initial_size: f32,
    pub velocity: Vec2,
}

pub fn spawn_spark(commands: &mut Commands, position: Vec3) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let speed = rng.gen_range(0.5..2.0);
    let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 0.6, 0.0, 0.8),
                custom_size: Some(Vec2::new(PIXEL_SIZE * 0.5, PIXEL_SIZE * 0.5)),
                ..default()
            },
            transform: Transform::from_xyz(
                position.x,
                position.y,
                position.z + 0.1
            ),
            ..default()
        },
        Effect {
            lifetime: rng.gen_range(0.2..0.5),
            initial_size: PIXEL_SIZE * 0.5,
            velocity,
        },
    ));
}

pub fn spawn_bubble(commands: &mut Commands, position: Vec3) {
    let mut rng = rand::thread_rng();
    let wobble = rng.gen_range(-0.5..0.5);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.7, 0.9, 1.0, 0.3),
                custom_size: Some(Vec2::new(PIXEL_SIZE * 0.7, PIXEL_SIZE * 0.7)),
                ..default()
            },
            transform: Transform::from_xyz(
                position.x,
                position.y,
                position.z + 0.1
            ),
            ..default()
        },
        Effect {
            lifetime: rng.gen_range(0.5..1.5),
            initial_size: PIXEL_SIZE * 0.7,
            velocity: Vec2::new(wobble, 1.0),
        },
    ));
}

// Système pour mettre à jour les effets
pub fn update_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut effects: Query<(Entity, &mut Effect, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut effect, mut transform, mut sprite) in effects.iter_mut() {
        effect.lifetime -= time.delta_seconds();

        // Mettre à jour la position
        transform.translation.x += effect.velocity.x;
        transform.translation.y += effect.velocity.y;

        // Faire disparaître progressivement
        let alpha = (effect.lifetime / 0.5).min(1.0);
        sprite.color.set_a(alpha);

        // Supprimer si la durée de vie est écoulée
        if effect.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}