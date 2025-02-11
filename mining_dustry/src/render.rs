// src/render.rs

use crate::particles::base::*;
use bevy::prelude::*;

pub fn render_particles(mut particles: Query<(&Particle, &mut Sprite)>) {
    for (particle, mut sprite) in particles.iter_mut() {
        let base_color = get_particle_color(particle.particle_type);

        // Ajouter des variations de couleur basées sur la profondeur
        let depth_factor = particle.light_level.max(0.2);
        sprite.color = Color::rgb(
            base_color.r() * depth_factor,
            base_color.g() * depth_factor,
            base_color.b() * depth_factor,
        );
    }
}

fn get_particle_color(particle_type: ParticleType) -> Color {
    match particle_type {
        ParticleType::Sand => Color::rgb(0.76, 0.7, 0.5),
        ParticleType::Water => Color::rgba(0.2, 0.5, 1.0, 0.8), // Semi-transparent
        ParticleType::Lava => {
            // Couleur de lave plus dynamique
            let flicker = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as f32
                * 0.01)
                .sin()
                * 0.1
                + 0.9;
            Color::rgb(1.0 * flicker, 0.3 * flicker, 0.0)
        }
        ParticleType::Fire => Color::rgb(1.0, 0.5, 0.0),
        ParticleType::Stone => Color::rgb(0.5, 0.5, 0.5),
        ParticleType::Steam => Color::rgb(0.8, 0.8, 0.8),
        ParticleType::Wood => Color::rgb(0.5, 0.3, 0.0),
        ParticleType::Oil => Color::rgb(0.2, 0.2, 0.2),
        ParticleType::Smoke => Color::rgb(0.5, 0.5, 0.5),
    }
}
