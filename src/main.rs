mod frame;
mod offset;
mod particle;
mod simulation;

use crate::frame::Frame;
use minifb::{Key, MouseButton, Window, WindowOptions};
use offset::Offset;
use particle::Particle;
use simulation::Simulation;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const LOGICAL_SCALE: usize = 5;

fn main() {
    let mut frame = Frame::new_with_scale(WIDTH, HEIGHT, LOGICAL_SCALE);

    let mut window = Window::new(
        "Particle Simulation",
        WIDTH * LOGICAL_SCALE,
        HEIGHT * LOGICAL_SCALE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit to max ~60 fps update rate
    window.set_target_fps(30);

    let mut simulation = Simulation::new(WIDTH, HEIGHT);

    let unique_particles = vec![Particle::sand(), Particle::water()];
    let mut index = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            index = (index + 1) % unique_particles.len();
        }

        if window.get_mouse_down(MouseButton::Left) {
            let opt = window.get_mouse_pos(minifb::MouseMode::Clamp);
            match opt {
                Some((x, y)) => {
                    let (log_x, log_y) = frame.real_pos_to_logical(x as usize, y as usize);
                    let offset = Offset::new(log_x as i32, log_y as i32);

                    simulation.add_particle(offset, unique_particles[index]);
                }
                None => {}
            }
        }

        // Simulate and draw the particles
        simulation.simulate_step();
        simulation.draw_to_frame(&mut frame);

        draw_ui_to_frame(&mut frame, &unique_particles[index]);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&frame.buffer, WIDTH * LOGICAL_SCALE, HEIGHT * LOGICAL_SCALE)
            .unwrap();
    }
}

fn draw_ui_to_frame(frame: &mut Frame, current_particle: &Particle) {
    let _ = frame.draw_pixel(1, 1, current_particle.color);
}
