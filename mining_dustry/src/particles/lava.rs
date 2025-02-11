// src/particles/lava.rs

use bevy::prelude::*;
use crate::particles::base::{Particle, ParticleType, PIXEL_SIZE, Grid};
use rand::random;

pub struct Lava;

impl Lava {
    pub fn update(
        transform: &mut Transform,
        grid: &mut Grid,
        x: usize,
        y: usize,
    ) -> bool {
        let mut moved = false;
        let mut fall_direction = Vec2::ZERO;

        if y > 0 && grid.cells[x][y-1].is_none() {
            fall_direction = Vec2::new(0.0, -1.0);
            moved = true;
        } else if y > 0 {
            let try_left = x > 0 && grid.cells[x-1][y-1].is_none();
            let try_right = x < grid.width-1 && grid.cells[x+1][y-1].is_none();

            if try_left && try_right {
                if random() {
                    fall_direction = Vec2::new(-1.0, -1.0);
                } else {
                    fall_direction = Vec2::new(1.0, -1.0);
                }
                moved = true;
            } else if try_left {
                fall_direction = Vec2::new(-1.0, -1.0);
                moved = true;
            } else if try_right {
                fall_direction = Vec2::new(1.0, -1.0);
                moved = true;
            }
        }

        if moved {
            transform.translation.x += fall_direction.x * PIXEL_SIZE;
            transform.translation.y += fall_direction.y * PIXEL_SIZE;
        }

        moved
    }
}
