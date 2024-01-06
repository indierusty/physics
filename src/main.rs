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
    let mut particle = Particle::new(vec2(100., 100.), vec2(0., 0.), vec2(0., 0.), 1.0, 10.0);
    let mut push_force;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(WHITE);

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
        let friction_force = generate_friction_force(particle.velocity, 0.5);
        particle.add_force(friction_force * PIXELS_PER_METER);
        // println!("{:?}", particle.velocity.normalize());

        particle.update();
        particle.draw();

        next_frame().await
    }
}
