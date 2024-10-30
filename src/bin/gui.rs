// Drawing to image was inspired by:
// Source: https://github.com/bluurryy/noise-functions-demo/blob/main/src/app.rs

use eframe::egui;
use particle_simulation::{particles::Sand, Color, Offset, Simulation};

const SIM_WIDTH: usize = 200;
const SIM_HEIGHT: usize = 200;
const ZOOM: f32 = 4.0;

fn color_to_color32(c: &Color) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

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
}

impl GUIParticleSim {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let simulation = Simulation::new(SIM_WIDTH, SIM_HEIGHT);

        GUIParticleSim {
            simulation,
            pixels: vec![egui::Color32::from_rgb(0, 0, 0); SIM_WIDTH * SIM_HEIGHT],
            texture: cc.egui_ctx.load_texture(
                "sim_view",
                egui::ColorImage::new([SIM_WIDTH, SIM_HEIGHT], egui::Color32::from_rgb(0, 0, 0)),
                egui::TextureOptions::NEAREST,
            ),
        }
    }
}

impl eframe::App for GUIParticleSim {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(ZOOM);

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

            let bg = egui::Color32::LIGHT_BLUE;
            for (i, opt) in self.simulation.particles_iter().enumerate() {
                match opt {
                    Some(p) => {
                        self.pixels[i] = color_to_color32(p.get_color());
                    }
                    None => {
                        self.pixels[i] = bg;
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
