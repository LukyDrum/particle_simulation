// MODS
mod frame;
mod offset;
mod particles;
mod simulation;
mod sprite;
mod test;
mod utility;

// IMPORTS
use std::time::SystemTime;

use crate::frame::Frame;
use minifb::{Key, MouseButton, Window, WindowOptions};
use offset::Offset;
use particles::{Fly, Mud, Oil, Particle, Rock, Sand, Smoke, Spark, Static, Water, Wood};
use simulation::Simulation;
use sprite::Sprite;
use utility::{draw_ui_to_frame, get_offsets_for_square};

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const LOGICAL_SCALE: usize = 3;
const INDICATOR_SIZE: usize = 10;
const BRUSH_SIZE: usize = 5;

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
    // Limit to max ~30 fps update rate
    window.set_target_fps(60);

    let mut simulation = Simulation::new(WIDTH, HEIGHT);
    // simulation.print_debug = true;

    let unique_particles = vec![
        Sand::new,
        Mud::new,
        Water::new,
        Rock::new,
        Oil::new,
        Fly::new,
        Spark::new,
        Wood::new,
        Smoke::new,
    ];
    let indicator_particles: Vec<Box<dyn Particle>> =
        unique_particles.iter().map(|p| p()).collect();
    let mut index = 0;

    let mut cur_time = SystemTime::now();
    let mut last_time = cur_time;
    let mut fps_counter = 0;
    let mut avg_fps = 0;

    let mut is_sim_running: bool = false;

    simulation.bg_color = 0xFFD1FFFC;
    // Load and insert sprite
    let fit_sprite = Sprite::load("assets/fit_pixel_blue.png");
    if let Ok(sprite) = fit_sprite {
        simulation.insert_sprite(sprite, &Offset::new(80, 50), |color| match color {
            0xFFFFFFFF => Static::new(color),
            _ => Water::with_color(color),
        });
    }

    let fit_sprite = Sprite::load("assets/fit_pixel.png");
    if let Ok(sprite) = fit_sprite {
        simulation.insert_sprite(sprite, &Offset::new(20, 50), |color| match color {
            0xFF000000 => Static::new(color),
            _ => Sand::with_color(color),
        });
    }

    // Print controls to terminal
    println!();
    println!("Controls:");
    println!("P: Pause/Resume simulation");
    println!("Space: Cycle particles");
    println!("LMB: Place particle");
    if !is_sim_running {
        println!("\nStarting paused!");
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cur_time = SystemTime::now();

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            index = (index + 1) % unique_particles.len();
            println!("Particle: {}", indicator_particles[index].get_name());
        }

        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            is_sim_running = !is_sim_running;
        }

        if window.get_mouse_down(MouseButton::Left) {
            let opt = window.get_mouse_pos(minifb::MouseMode::Clamp);
            match opt {
                Some((x, y)) => {
                    let (log_x, log_y) = frame.real_pos_to_logical(x as usize, y as usize);
                    let center = Offset::new(log_x as i32, log_y as i32);

                    if BRUSH_SIZE == 1 {
                        simulation.add_particle(&center, unique_particles[index]());
                    } else {
                        for off in get_offsets_for_square(&center, BRUSH_SIZE) {
                            simulation.add_particle(&off, unique_particles[index]());
                        }
                    }
                }
                None => {}
            }
        } else if window.get_mouse_down(MouseButton::Right) {
            let opt = window.get_mouse_pos(minifb::MouseMode::Clamp);
            match opt {
                Some((x, y)) => {
                    let (log_x, log_y) = frame.real_pos_to_logical(x as usize, y as usize);
                    let center = Offset::new(log_x as i32, log_y as i32);

                    for off in get_offsets_for_square(&center, BRUSH_SIZE) {
                        simulation.remove_particle(&off);
                    }
                }
                None => {}
            }
        }

        // Simulate and draw the particles
        if is_sim_running {
            simulation.simulate_step();
        }
        simulation.draw_to_frame(&mut frame);

        // Print FPS
        fps_counter += 1;
        if cur_time.duration_since(last_time).unwrap().as_secs_f32() >= 1.0 {
            avg_fps = fps_counter;
            fps_counter = 0;
            last_time = cur_time;
        }
        // println!("FPS: {}", avg_fps);

        draw_ui_to_frame(&mut frame, &indicator_particles[index], INDICATOR_SIZE);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&frame.buffer, WIDTH * LOGICAL_SCALE, HEIGHT * LOGICAL_SCALE)
            .unwrap();
    }
}
