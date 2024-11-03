// Drawing to image was inspired by:
// Source: https://github.com/bluurryy/noise-functions-demo/blob/main/src/app.rs

use eframe::egui;
use particle_simulation::{
    particles::{
        constants::CELL_DEFAULT_PRESSURE, Fly, Mud, Oil, Particle, Rock, Sand, Smoke, Spark, Vapor,
        Water, Wood,
    },
    utility::get_offsets_for_square,
    Color, Offset, Simulation,
};

const SIM_WIDTH: usize = 200;
const SIM_HEIGHT: usize = 200;
const ZOOM: f32 = 3.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ViewMode {
    Normal,
    Pressure,
}

fn color_to_color32(c: &Color) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Particle Simulation",
        native_options,
        Box::new(|cc| Ok(Box::new(GUIParticleSim::new(cc)))),
    );
}

struct GUIParticleSim {
    simulation: Simulation,
    texture: egui::TextureHandle,
    view_rect: egui::Rect,
    /// Function to call to create a new particle of type
    particles_new_functions: Vec<fn() -> Box<dyn Particle>>,
    /// Preview of each particle type
    preview_particles: Vec<Box<dyn Particle>>,
    selected_particle_index: usize,
    /// The resulting are will be this value squared
    brush_size: u32,
    view_mode: ViewMode,
}

impl GUIParticleSim {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let simulation = Simulation::new(SIM_WIDTH, SIM_HEIGHT);

        // Function to call to create a new particle of type
        let particles_new_functions = vec![
            Sand::new,
            Water::new,
            Rock::new,
            Mud::new,
            Oil::new,
            Wood::new,
            Spark::new,
            Fly::new,
            Smoke::new,
            Vapor::new,
        ];
        // Create preview particles by mapping the new functions
        let preview_particles = particles_new_functions.iter().map(|f| f()).collect();

        GUIParticleSim {
            simulation,
            texture: cc.egui_ctx.load_texture(
                "sim_view",
                egui::ColorImage::new([SIM_WIDTH, SIM_HEIGHT], egui::Color32::from_rgb(0, 0, 0)),
                egui::TextureOptions::NEAREST,
            ),
            view_rect: egui::Rect::ZERO,
            particles_new_functions,
            preview_particles,
            selected_particle_index: 0,
            brush_size: 4,
            view_mode: ViewMode::Normal,
        }
    }
}

impl eframe::App for GUIParticleSim {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(ZOOM);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Check for mouse presses inside the view rect
            ui.input(|input| {
                // Check for left mouse button
                if input.pointer.primary_down() {
                    // Get the position
                    let pos = input.pointer.interact_pos().unwrap();
                    // Check if it is inside the RECT
                    if self.view_rect.contains(pos) {
                        let pos_relative_to_view = pos - self.view_rect.left_top();
                        let center = Offset::new(
                            pos_relative_to_view.x as i32,
                            pos_relative_to_view.y as i32,
                        );

                        // Spawn particles in a square around the center with length of side equal to brush_size
                        for off in get_offsets_for_square(&center, self.brush_size) {
                            self.simulation.add_particle(
                                &off,
                                self.particles_new_functions[self.selected_particle_index](), // Call the new function of the currently selected particle
                            );
                        }
                    }
                }

                // Check for right mouse button
                if input.pointer.secondary_down() {
                    // Get the position
                    let pos = input.pointer.interact_pos().unwrap();
                    // Check if it is inside the RECT
                    if self.view_rect.contains(pos) {
                        let pos_relative_to_view = pos - self.view_rect.left_top();
                        let center = Offset::new(
                            pos_relative_to_view.x as i32,
                            pos_relative_to_view.y as i32,
                        );

                        // Spawn particles in a square around the center with length of side equal to brush_size
                        for off in get_offsets_for_square(&center, self.brush_size) {
                            self.simulation.remove_particle(&off);
                        }
                    }
                }
            });

            let bg = egui::Color32::LIGHT_BLUE;

            // Map simulation based on view mode
            let pixels: Vec<egui::Color32> = match self.view_mode {
                ViewMode::Normal => self
                    .simulation
                    .cells_iter()
                    .map(|cell| match cell.get_particle() {
                        Some(p) => color_to_color32(p.get_color()),
                        None => bg,
                    })
                    .collect(),
                ViewMode::Pressure => self
                    .simulation
                    .cells_iter()
                    .map(|cell| {
                        if !cell.is_empty() && cell.get_pressure() == CELL_DEFAULT_PRESSURE {
                            egui::Color32::from_rgb(242, 242, 242)
                        } else {
                            egui::Color32::from_rgb((cell.get_pressure() * 3).min(255) as u8, 0, 0)
                        }
                    })
                    .collect(),
            };

            // Draw pixels to texture
            self.texture.set(
                egui::ColorImage {
                    size: [self.simulation.width(), self.simulation.height()],
                    pixels,
                },
                egui::TextureOptions::NEAREST,
            );

            // UI ELEMENTS

            // Add label
            ui.add(egui::Label::new("Particle Simulation"));

            // Make 2 columns, one for simulation view, second for buttons
            ui.columns(2, |cols| {
                // COLUMN 0
                // Paint the texture to ui
                let size = self.texture.size_vec2();
                let sized_texture = egui::load::SizedTexture::new(self.texture.id(), size);
                let img = egui::Image::new(sized_texture);
                let img_response = cols[0].add(img);
                // Set the rect of the image as view rect
                self.view_rect = img_response.rect;

                // COLUMN 1
                // Add brush size slider
                cols[1].add(egui::Slider::new(&mut self.brush_size, 1..=20).text("Brush size"));

                egui::ComboBox::from_label("View mode")
                    .selected_text(format!("{:?}", self.view_mode))
                    .show_ui(&mut cols[1], |ui| {
                        ui.selectable_value(&mut self.view_mode, ViewMode::Normal, "Normal");
                        ui.selectable_value(&mut self.view_mode, ViewMode::Pressure, "Pressure");
                    });

                // Add label for particles
                cols[1].add(egui::Label::new("Particles"));

                egui::Grid::new("Particle buttons grid").show(&mut cols[1], |ui_col| {
                    let mut counter = 0;
                    // Add particle buttons
                    for (index, preview) in self.preview_particles.iter().enumerate() {
                        let button = egui::Button::new(preview.get_name())
                            .fill(color_to_color32(preview.get_color()))
                            .selected(index == self.selected_particle_index);
                        let response = ui_col.add(button);
                        // If clicked, set the selected particle index
                        if response.clicked() {
                            self.selected_particle_index = index;
                        }

                        // Increase counter and potentialy add end row
                        counter += 1;
                        if counter % 2 == 0 {
                            ui_col.end_row();
                        }
                    }
                });
            });
        });

        // Step the simulation
        self.simulation.simulate_step();
        // Request repaint again
        ctx.request_repaint();
    }
}
