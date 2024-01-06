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

#[macroquad::main(window_conf)]
async fn main() {
    // drag_and_gravity().await;
    // friction().await;
    // gravitation().await;
    spring().await;
}

use physics::forces::*;
use physics::particle::*;
use physics::PIXELS_PER_METER;

async fn spring() {
    let mut a = Particle::new(vec2(100., 100.), Vec2::ZERO, Vec2::ZERO, 1., 10.);
    let mut b = Particle::new(vec2(300., 100.), Vec2::ZERO, Vec2::ZERO, 1., 10.);
    let mut c = Particle::new(vec2(300., 300.), Vec2::ZERO, Vec2::ZERO, 1., 10.);
    let mut d = Particle::new(vec2(100., 300.), Vec2::ZERO, Vec2::ZERO, 1., 10.);

    let mut push_force;

    let mut springs: Vec<(Vec2, Vec2)> = vec![];

    const REST_LEN: f32 = 200.;
    const K: f32 = 1000.0;

    loop {
        springs.clear();
        if is_key_pressed(KeyCode::Space) {
            break;
        }

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

        // a -> b
        let spring_force = generate_spring_force(a.position, b.position, REST_LEN, K);
        a.add_force(spring_force);
        b.add_force(-spring_force);
        springs.push((a.position, b.position));

        // b -> c
        let spring_force = generate_spring_force(b.position, c.position, REST_LEN, K);
        b.add_force(spring_force);
        c.add_force(-spring_force);
        springs.push((b.position, c.position));

        // c -> d
        let spring_force = generate_spring_force(c.position, d.position, REST_LEN, K);
        c.add_force(spring_force);
        d.add_force(-spring_force);
        springs.push((c.position, d.position));

        // d -> a
        let spring_force = generate_spring_force(d.position, a.position, REST_LEN, K);
        d.add_force(spring_force);
        a.add_force(-spring_force);
        springs.push((d.position, a.position));

        // a -> c
        let spring_force = generate_spring_force(a.position, c.position, REST_LEN, K);
        a.add_force(spring_force);
        c.add_force(-spring_force);
        springs.push((a.position, c.position));

        // b -> d
        let spring_force = generate_spring_force(b.position, d.position, REST_LEN, K);
        b.add_force(spring_force);
        d.add_force(-spring_force);
        springs.push((b.position, d.position));

        // TEMP:
        a.add_force(push_force * PIXELS_PER_METER * 50.);

        for x in [&mut a, &mut b, &mut c, &mut d] {
            // gravity
            let weight = vec2(0.0, x.mass * 9.8 * PIXELS_PER_METER);
            x.add_force(weight);

            // // drag
            let drag = generate_drag_force(x.velocity, 0.003);
            x.add_force(drag);

            x.update();
        }

        clear_background(WHITE);
        // draw spring
        for spring in &springs {
            draw_line(
                spring.0.x, spring.0.y, spring.1.x, spring.1.y, 3.0, DARKGRAY,
            );
        }
        // draw particle
        for x in [&mut a, &mut b, &mut c, &mut d] {
            x.draw();
        }

        next_frame().await
    }
}
