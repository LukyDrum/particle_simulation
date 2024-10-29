// Source: https://github.com/bluurryy/noise-functions-demo/blob/main/src/app.rs

use eframe::egui;
use particle_simulation::{particles::Sand, Offset, Simulation};

const SIM_WIDTH: usize = 200;
const SIM_HEIGHT: usize = 200;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(GUIParticleSim::new(cc)))),
    );
}

struct GUIParticleSim {
    simulation: Simulation,
    pixels: Vec<egui::Color32>,
    texture: egui::TextureHandle,
    index: usize,
}

impl GUIParticleSim {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let simulation = Simulation::new(SIM_WIDTH, SIM_HEIGHT);

        GUIParticleSim {
            simulation,
            pixels: vec![egui::Color32::from_rgb(0, 0, 0); 200 * 200],
            texture: cc.egui_ctx.load_texture(
                "sim_view",
                egui::ColorImage::new([200, 200], egui::Color32::from_rgb(0, 0, 0)),
                egui::TextureOptions::NEAREST,
            ),
            index: 0,
        }
    }
}

impl eframe::App for GUIParticleSim {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Define a rect at a position with size
            // This will hold the texture with the visual output of the simulation
            let rect = egui::Rect::from_min_max(
                egui::Pos2 { x: 0.0, y: 0.0 },
                egui::Pos2 {
                    x: SIM_WIDTH as f32,
                    y: SIM_HEIGHT as f32,
                },
            );

            // Check for button presses inside the rect
            ui.input(|input| {
                // Check for left mouse button
                if input.pointer.primary_down() {
                    // Get the position
                    let pos = input.pointer.interact_pos().unwrap();
                    // Check if it is inside the RECT
                    if rect.contains(pos) {
                        self.simulation
                            .add_particle(&Offset::new(pos.x as i32, pos.y as i32), Sand::new());
                    }
                }
            });

            let black = egui::Color32::from_rgb(255, 255, 255);
            for (i, opt) in self.simulation.particles.iter().enumerate() {
                match opt {
                    Some(p) => {
                        self.pixels[i] = u32_to_color32(p.get_color());
                    }
                    None => {
                        self.pixels[i] = black;
                    }
                }
            }
            // Draw pixels to texture
            self.texture.set(
                egui::ColorImage {
                    size: [SIM_WIDTH, SIM_HEIGHT],
                    pixels: self.pixels.clone(),
                },
                egui::TextureOptions::NEAREST,
            );

            // Paint the texture to rect
            let size = self.texture.size_vec2();
            let sized_texture = egui::load::SizedTexture::new(self.texture.id(), size);
            egui::Image::new(sized_texture).paint_at(ui, rect);
        });

        // Step the simulation
        self.simulation.simulate_step();
        // Request repaint again
        ctx.request_repaint();
    }
}

fn u32_to_color32(color: u32) -> egui::Color32 {
    let r = color & 0x00FF0000;
    let g = color & 0x0000FF00;
    let b = color & 0x000000FF;

    egui::Color32::from_rgb(r as u8, g as u8, b as u8)
}
