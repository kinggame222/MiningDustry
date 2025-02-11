// src/particles/base.rs

use bevy::prelude::*;

pub const PIXEL_SIZE: f32 = 2.0;

#[derive(Resource)]
pub struct Grid {
    pub cells: Vec<Vec<Option<Entity>>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum ParticleType {
    Sand,
    Water,
    Stone,
    Fire,
    Steam,
    Wood,
    Oil,
    Smoke,
    Lava,
}

#[derive(Component)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub velocity: Vec2,
    pub temperature: f32,
    pub life: f32,
    pub light_level: f32,
}

#[derive(Component)]
pub struct ParticleLight {
    pub intensity: f32,
    pub color: Color,
}
