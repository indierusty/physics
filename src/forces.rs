use crate::particle::Particle;
use macroquad::prelude::*;

pub fn generate_drag_force(velocity: Vec2, k: f32) -> Vec2 {
    let mut drag_force = vec2(0., 0.);

    if velocity.length_squared() > 0. {
        let drag_direction = velocity.normalize() * -1.;
        let drag_magnitude = k * velocity.length_squared();
        drag_force = drag_direction * drag_magnitude;
    }

    drag_force
}

pub fn generate_friction_force(velocity: Vec2, k: f32) -> Vec2 {
    if velocity == vec2(0., 0.) {
        return vec2(0., 0.);
    }

    let direction = velocity.normalize() * -1.;
    let magnitude = k;

    direction * magnitude
}

pub fn generate_gravitional_force(
    a: Particle,
    b: Particle,
    G: f32,
    mindistance: f32,
    maxdistance: f32,
) -> Vec2 {
    let a2b = b.position - a.position;
    let mass_squared = a.mass * b.mass;
    let distance_squared = a2b.length_squared().clamp(mindistance, maxdistance);
    let direction = a2b.normalize();

    G * mass_squared / distance_squared * direction
}

pub fn generate_spring_force(a: Vec2, b: Vec2, restlen: f32, k: f32) -> Vec2 {
    let a_to_b = a - b;
    let displacement = a_to_b.length() - restlen;

    let spring_direction = a_to_b.normalize();
    let spring_magnitude = -k * displacement;

    spring_direction * spring_magnitude
}
