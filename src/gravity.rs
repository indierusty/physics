use macroquad::prelude::*;

use physics::forces::*;
use physics::particle::*;
use physics::PIXELS_PER_METER;

fn window_conf() -> Conf {
    Conf {
        window_title: "Physics".to_owned(),
        fullscreen: false,
        window_width: 720,
        window_height: 520,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let small = Particle::new(
        vec2(100., screen_height() / 2.),
        vec2(0., 0.),
        vec2(0., 0.),
        5.0,
        10.0,
    );
    let big = Particle::new(
        vec2(screen_width() / 2., screen_height() / 2.),
        vec2(0., 0.),
        vec2(0., 0.),
        50.0,
        20.0,
    );
    let mut particles = vec![small, big];
    let mut push_force;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let particle = Particle::new(
                vec2(mouse_position().0, mouse_position().1),
                vec2(0., 0.),
                vec2(0., 0.),
                1.0,
                10.,
            );
            particles.push(particle);
        }

        for particle in &mut particles {
            // push force to be controlled by keyboard
            if is_key_down(KeyCode::Left) {
                push_force = vec2(-1., 0.);
            } else if is_key_down(KeyCode::Right) {
                push_force = vec2(1., 0.);
            } else if is_key_down(KeyCode::Up) {
                push_force = vec2(0., -1.);
            } else if is_key_down(KeyCode::Down) {
                push_force = vec2(0., 1.);
            } else {
                push_force = vec2(0., 0.);
            }

            particle.add_force(push_force * PIXELS_PER_METER * 15.0);

            particle.update();
        }

        for a in 0..particles.len() {
            for b in 0..particles.len() {
                if a != b {
                    let gravitational_force =
                        generate_gravitional_force(particles[a], particles[b], 400., 10.0, 1000.);
                    particles[a].add_force(gravitational_force);
                    particles[b].add_force(-gravitational_force);
                }
            }
        }

        clear_background(WHITE);

        for particle in &particles {
            particle.draw();
        }

        draw_text(
            "Press Arrow keys to add force to small particle",
            20.,
            20.,
            25.,
            DARKGRAY,
        );

        next_frame().await
    }
}
