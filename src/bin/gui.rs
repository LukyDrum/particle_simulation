// Source: https://github.com/bluurryy/noise-functions-demo/blob/main/src/app.rs

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(GUIParticleSim::new(cc)))),
    );
}

struct GUIParticleSim {
    pixels: Vec<egui::Color32>,
    texture: egui::TextureHandle,
    index: usize,
}

impl GUIParticleSim {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        GUIParticleSim {
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
            ui.heading("Particle simulation based on CA");

            self.pixels[self.index] = egui::Color32::from_rgb(255, 255, 255);
            // Draw pixels to texture
            self.texture.set(
                egui::ColorImage {
                    size: [200, 200],
                    pixels: self.pixels.clone(),
                },
                egui::TextureOptions::NEAREST,
            );

            let size = self.texture.size_vec2();
            let sized_texture = egui::load::SizedTexture::new(self.texture.id(), size);
            ui.add(egui::Image::new(sized_texture).fit_to_exact_size(size));
        });

        self.index = (self.index + 1) % (200 * 200);
    }
}
