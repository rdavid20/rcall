use eframe::egui;
use eframe::egui::{ColorImage, TextureHandle};
use rusqlite::Connection;

use crate::database;

pub struct GUI {
    user_input: String,
    results: Vec<String>,
    db_conn: Connection,
    selected_index: Option<usize>,
    texture: Option<TextureHandle>,
}

impl GUI {
    pub fn new(db_conn: Connection) -> Self {
        Self {
            user_input: String::new(),
            results: vec![],
            db_conn,
            selected_index: None,
            texture: None,
        }
    }

    fn perform_search(&mut self) {
        if self.user_input.trim().is_empty() {
            return;
        }

        let result = database::search_images(&self.db_conn, &self.user_input);

        match result {
            Ok(vec) => {
                self.results = vec;
            }

            Err(e) => {
                self.results.clear();
                eprintln!("Error: {}", e);
            }
        }
    }

    fn load_image(&mut self, ctx: &egui::Context, path: &str) {
        let image = image::open(path).expect("Failed to load image");

        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        self.texture =
            Some(ctx.load_texture("preview", color_image, egui::TextureOptions::default()));
    }
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RCall Search Engine!");

            ui.label("Input Keyword:");

            ui.horizontal(|ui| {
                let res = ui.text_edit_singleline(&mut self.user_input);

                if res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.perform_search();
                }

                if ui.button("Search").clicked() {
                    self.perform_search();
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                egui::ScrollArea::vertical()
                    .max_width(600.0)
                    .max_height(2000.0)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            if self.results.is_empty() {
                                ui.label("No Entries Found");
                            } else {
                                let mut new_selection: Option<usize> = None;
                                for (i, file) in self.results.iter().enumerate() {
                                    let is_selected = self.selected_index == Some(i);
                                    if ui.selectable_label(is_selected, file).clicked() {
                                        new_selection = Some(i);
                                    }
                                }
                                if let Some(i) = new_selection {
                                    self.selected_index = Some(i);
                                    let path = self.results[i].clone();
                                    self.load_image(ctx, &path);
                                }
                            }
                        });
                    });

                ui.separator();

                ui.vertical(|ui| {
                    if let Some(selected) = self.selected_index {
                        if selected < self.results.len() {
                            if let Some(tex) = &self.texture {
                                ui.image(tex);
                            }
                        } else {
                            self.selected_index = None;
                        }
                    } else {
                        ui.label("No file selected");
                    }
                })
            });
        });
    }
}
