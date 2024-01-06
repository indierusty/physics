pub mod forces;
pub mod particle;

use macroquad::prelude::{vec2, Vec2};

pub const PIXELS_PER_METER: f32 = 50.;
pub const PARTICLE_RADIUS: f32 = 10.0;
pub const GRAVITY: Vec2 = vec2(0., 9.8);
