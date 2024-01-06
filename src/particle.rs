use crate::{GRAVITY, PIXELS_PER_METER};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    acceleration: Vec2,
    sum_force: Vec2,
    pub mass: f32,
    inverse_mass: f32,
    radius: f32,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, mass: f32, radius: f32) -> Self {
        let inverse_mass = if mass != 0. { 1.0 / mass } else { 0. };

        Self {
            position,
            velocity,
            acceleration,
            mass,
            inverse_mass,
            radius,
            sum_force: vec2(0., 0.),
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }

    fn integrate(&mut self) {
        // NOTE: Order of integration is important -> acceleration -> velocity -> position
        self.acceleration = self.sum_force * self.inverse_mass;
        self.velocity += self.acceleration * get_frame_time();
        self.position += self.velocity * get_frame_time();

        self.clear_force();
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.sum_force += force;
    }

    fn clear_force(&mut self) {
        self.sum_force = vec2(0., 0.);
    }

    pub fn add_gravity(&mut self) {
        self.add_force(GRAVITY * PIXELS_PER_METER * self.mass);
    }

    pub fn update(&mut self) {
        self.integrate();

        if self.position.y > screen_height() - self.radius {
            self.position.y = screen_height() - self.radius;
            self.velocity.y *= -0.9;
        } else if self.position.y < 0.0 + self.radius {
            self.position.y = self.radius;
            self.velocity.y *= -0.9;
        }
        if self.position.x > screen_width() - self.radius {
            self.position.x = screen_width() - self.radius;
            self.velocity.x *= -0.9;
        } else if self.position.x < 0.0 + self.radius {
            self.position.x = self.radius;
            self.velocity.x *= -0.9;
        }
    }
}
