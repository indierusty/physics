use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Physics".to_owned(),
        fullscreen: false,
        window_width: 720,
        window_height: 520,
        ..Default::default()
    }
}
use physics::forces::*;
use physics::particle::*;
use physics::PIXELS_PER_METER;

#[macroquad::main(window_conf)]
async fn main() {
    let small = Particle::new(vec2(100., 100.), vec2(0., 0.), vec2(0., 0.), 1.0, 10.0);
    let big = Particle::new(vec2(200., 100.), vec2(0., 0.), vec2(0., 0.), 3.0, 12.0);

    let mut particles = vec![small, big];
    let mut push_force;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let particle = Particle::new(
                vec2(mouse_position().0, mouse_position().1),
                vec2(0., 0.),
                vec2(0., 0.),
                1.0,
                10.,
            );
            particles.push(particle);
        }

        clear_background(WHITE);

        // fluid
        draw_rectangle(
            0.,
            screen_height() / 2.,
            screen_width(),
            screen_height() / 2.,
            Color::new(1., 0., 0., 0.3),
        );

        for particle in &mut particles {
            // gravity
            particle.add_gravity();
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

            // drag force
            if particle.position.y > screen_height() / 2. {
                let drag_force = generate_drag_force(particle.velocity, 0.05);
                particle.add_force(drag_force);
            } else {
                // wind force
                particle.add_force(vec2(0.5 * PIXELS_PER_METER, 0.));
            }

            particle.update();
            particle.draw();
        }

        next_frame().await
    }
}
