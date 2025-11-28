use eframe::egui;
use image::{Rgba, RgbaImage};
use svg::node::element::Rectangle;
use rand::Rng;
use svg::Document;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1400.0, 900.0]),
        ..Default::default()
    };

    eframe::run_native(
        "FraCantor",
        options,
        Box::new(|cc| {
            customize_style(&cc.egui_ctx);
            Ok(Box::new(FraCantor::default()))
        }),
    )
}

fn customize_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Fond global uniforme
    style.visuals.panel_fill = egui::Color32::from_rgb(249, 250, 251); // #F9FAFB

    // Widgets
    style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(255, 255, 255);
    style.visuals.widgets.noninteractive.weak_bg_fill = egui::Color32::from_rgb(255, 255, 255);
    style.visuals.widgets.noninteractive.bg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 200, 200));

    // Boutons non actifs
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(255, 255, 255);
    style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(255, 255, 255);
    style.visuals.widgets.inactive.bg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 200, 200));
    style.visuals.widgets.inactive.fg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(189, 0, 0));

    // Boutons survolés
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(255, 245, 245);
    style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(255, 245, 245);
    style.visuals.widgets.hovered.bg_stroke =
        egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 0, 0));
    style.visuals.widgets.hovered.fg_stroke =
        egui::Stroke::new(1.5, egui::Color32::from_rgb(189, 0, 0));

    // Boutons actifs
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(255, 235, 235);
    style.visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(255, 235, 235);
    style.visuals.widgets.active.bg_stroke =
        egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 0, 0));
    style.visuals.widgets.active.fg_stroke =
        egui::Stroke::new(2.0, egui::Color32::from_rgb(189, 0, 0));

    // Sliders
    style.visuals.selection.bg_fill = egui::Color32::from_rgb(255, 0, 0);
    style.visuals.selection.stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 0, 0));
    style.visuals.slider_trailing_fill = true;

    // Coins arrondis
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(8.0);

    // Couleur du texte
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(50, 50, 50));

    ctx.set_style(style);
}

struct FraCantor {
    iterations: u32,
    zoom: f32,
    selected_color: usize,
    texture: Option<egui::TextureHandle>,
    colors: Vec<Rgba<u8>>,
}

impl Default for FraCantor {
    fn default() -> Self {
        Self {
            iterations: 1,
            zoom: 1.0,
            selected_color: 0,
            texture: None,
            colors: vec![
                Rgba([239, 71, 111, 255]),   // rose/rouge flashy
                Rgba([255, 209, 102, 255]),  // jaune pastel
                Rgba([6, 214, 160, 255]),    // vert menthe
                Rgba([17, 138, 178, 255]),   // bleu vif
                Rgba([155, 111, 242, 255]),  // violet
            ],
        }
    }
}

impl eframe::App for FraCantor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(249, 250, 251)))
            .show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    // COLONNE GAUCHE (Sidebar) - Largeur fixe
                    ui.allocate_ui(egui::vec2(320.0, ui.available_height()), |ui| {
                        ui.add_space(20.0);

                        ui.horizontal(|ui| {
                            ui.add_space(12.0); // Marge gauche
                            
                            // DIMENSIONS FIXES POUR TOUTES LES CARTES
                            ui.vertical(|ui| {
                                let card_width = 296.0; // Largeur fixe
                                let card_height = 112.0; // Hauteur fixe
                                
                                // Carte titre
                                ui.allocate_ui(egui::vec2(card_width, 0.0), |ui| {
                                    egui::Frame::none()
                                        .fill(egui::Color32::from_rgb(249, 237, 237))
                                        .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 0, 0)))
                                        .rounding(12.0)
                                        .inner_margin(egui::Margin::symmetric(20.0, 16.0))
                                        .show(ui, |ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.label(
                                                    egui::RichText::new("FraCantor")
                                                        .size(32.0)
                                                        .color(egui::Color32::from_rgb(189, 0, 0))
                                                        .strong(),
                                                );
                                            });
                                        });
                                });

                                ui.add_space(18.0);

                                // TOUTES les boîtes avec EXACTEMENT les mêmes dimensions
                                ui.allocate_ui(egui::vec2(card_width, card_height), |ui| {
                                    self.render_parameters_section(ui);
                                });
                                ui.add_space(16.0);

                                ui.allocate_ui(egui::vec2(card_width, card_height), |ui| {
                                    self.render_presets_section(ui);
                                });
                                ui.add_space(16.0);

                                ui.allocate_ui(egui::vec2(card_width, card_height), |ui| {
                                    self.render_colors_section(ui);
                                });
                                ui.add_space(16.0);

                                ui.allocate_ui(egui::vec2(card_width, card_height), |ui| {
                                    self.render_export_section(ui);
                                });
                                ui.add_space(16.0);

                                ui.allocate_ui(egui::vec2(card_width, card_height), |ui| {
                                    self.render_stats_section(ui);
                                });
                            });
                            
                            ui.add_space(24.0); // Marge droite
                        });
                    });

                    // SÉPARATION VERTICALE VISIBLE
                    ui.add_space(8.0);
                    let separator_rect = ui.allocate_space(egui::vec2(1.0, ui.available_height()));
                    ui.painter().rect_filled(
                        separator_rect.1,
                        0.0,
                        egui::Color32::from_rgb(255, 220, 220),
                    );
                    ui.add_space(8.0);

                    // COLONNE DROITE (Zone principale)
                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0);

                        let available = ui.available_size();
                        let frame_size = available.x.min(available.y - 60.0) - 60.0;

                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(252, 252, 252))
                            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 200, 200)))
                            .rounding(16.0)
                            .inner_margin(40.0)
                            .show(ui, |ui| {
                                ui.set_width(frame_size);
                                ui.set_height(frame_size);

                                ui.centered_and_justified(|ui| {
                                    self.render_fractal(ui, ctx);
                                });
                            });
                    });
                });
            });
    }
}

impl FraCantor {
    fn render_parameters_section(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(249, 237, 237))
            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 0, 0)))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_size(egui::vec2(available.x, available.y));
                ui.set_max_size(egui::vec2(available.x, available.y));
                
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("│")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(255, 0, 0))
                            .strong(),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Paramètres")
                            .size(17.0)
                            .color(egui::Color32::from_rgb(40, 40, 40))
                            .strong(),
                    );
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Itérations");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(self.iterations.to_string());
                    });
                });

                ui.add(egui::Slider::new(&mut self.iterations, 0..=6).show_value(false));

                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.label("Zoom");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("x{}", self.zoom as i32));
                    });
                });

                ui.add(egui::Slider::new(&mut self.zoom, 1.0..=15.0).show_value(false));
            });
    }

    fn render_presets_section(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(249, 237, 237))
            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 0, 0)))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_size(egui::vec2(available.x, available.y));
                ui.set_max_size(egui::vec2(available.x, available.y));
                
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("│")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(255, 0, 0))
                            .strong(),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Présets")
                            .size(17.0)
                            .color(egui::Color32::from_rgb(40, 40, 40))
                            .strong(),
                    );
                });

                ui.add_space(8.0);

                ui.vertical_centered(|ui| {
                    egui::Grid::new("presets_grid")
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            let button_size = egui::vec2(120.0, 28.0);

                            if ui.add_sized(button_size, egui::Button::new("1 itération")).clicked() {
                                self.iterations = 1;
                            }

                            if ui.add_sized(button_size, egui::Button::new("2 itérations")).clicked() {
                                self.iterations = 2;
                            }

                            ui.end_row();

                            if ui.add_sized(button_size, egui::Button::new("5 itérations")).clicked() {
                                self.iterations = 5;
                            }

                            if ui.add_sized(button_size, egui::Button::new("Mystère")).clicked() {
                                let mut rng = rand::rng();
                                self.iterations = rng.random_range(0..=6);
                                self.zoom = 1.0;
                            }
                        });
                });
            });
    }

    fn render_colors_section(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(249, 237, 237))
            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 0, 0)))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_size(egui::vec2(available.x, available.y));
                ui.set_max_size(egui::vec2(available.x, available.y));
                
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("│")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(255, 0, 0))
                            .strong(),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Couleurs")
                            .size(17.0)
                            .color(egui::Color32::from_rgb(40, 40, 40))
                            .strong(),
                    );
                });

                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    for (i, color) in self.colors.iter().enumerate() {
                        let size = egui::vec2(45.0, 45.0);
                        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

                        let color32 = egui::Color32::from_rgba_unmultiplied(
                            color[0], color[1], color[2], color[3],
                        );

                        ui.painter().rect_filled(rect, 6.0, color32);

                        if i == self.selected_color {
                            ui.painter().rect_stroke(
                                rect,
                                6.0,
                                egui::Stroke::new(3.0, egui::Color32::from_rgb(255, 255, 255)),
                            );
                        }

                        if response.clicked() {
                            self.selected_color = i;
                        }

                        ui.add_space(4.0);
                    }
                });
            });
    }

    fn render_export_section(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(249, 237, 237))
            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 0, 0)))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_size(egui::vec2(available.x, available.y));
                ui.set_max_size(egui::vec2(available.x, available.y));
                
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("│")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(255, 0, 0))
                            .strong(),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Export")
                            .size(17.0)
                            .color(egui::Color32::from_rgb(40, 40, 40))
                            .strong(),
                    );
                });

                ui.add_space(8.0);

                ui.vertical_centered(|ui| {
                    egui::Grid::new("export_grid")
                        .spacing([8.0, 6.0])
                        .show(ui, |ui| {
                            let button_size = egui::vec2(120.0, 24.0);

                            if ui.add_sized(button_size, egui::Button::new("PNG")).clicked() {
                                self.export_image("cantor.png");
                            }

                            if ui.add_sized(button_size, egui::Button::new("JPEG")).clicked() {
                                self.export_image("cantor.jpg");
                            }

                            ui.end_row();

                            if ui.add_sized(button_size, egui::Button::new("SVG")).clicked() {
                                self.export_svg("cantor.svg");
                            }

                            if ui.add_sized(button_size, egui::Button::new("GIF")).clicked() {
                                println!("Export GIF non implémenté");
                            }
                        });
                });
            });
    }

    fn render_stats_section(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(249, 237, 237))
            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 0, 0)))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_min_size(egui::vec2(available.x, available.y));
                ui.set_max_size(egui::vec2(available.x, available.y));
                
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("│")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(255, 0, 0))
                            .strong(),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Stats")
                            .size(17.0)
                            .color(egui::Color32::from_rgb(40, 40, 40))
                            .strong(),
                    );
                });

                ui.add_space(12.0);

                let num_squares = 4_u32.pow(self.iterations);

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Carrés :")
                            .color(egui::Color32::from_rgb(189, 0, 0))
                            .strong(),
                    );
                    ui.label(num_squares.to_string());
                });
            });
    }

    fn render_fractal(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let available_size = ui.available_size();
        let size = available_size.min_elem() as u32;

        let image = self.generate_cantor_image(size);
        let color_image = self.rgba_to_color_image(&image);

        let texture = self.texture.get_or_insert_with(|| {
            ctx.load_texture("cantor", color_image.clone(), egui::TextureOptions::default())
        });

        texture.set(color_image, egui::TextureOptions::NEAREST);

        ui.image(&*texture);
    }

    fn generate_cantor_image(&self, size: u32) -> RgbaImage {
        let mut image = RgbaImage::from_pixel(size, size, Rgba([255, 255, 255, 255]));
        let color = self.colors[self.selected_color];

        let view_size = size as f32 / self.zoom;
        let view_start_x = 0.0;
        let view_start_y = 0.0;

        let mut full_image = RgbaImage::from_pixel(size, size, Rgba([255, 255, 255, 255]));
        self.draw_cantor_square(&mut full_image, 0, 0, size, self.iterations, color);

        for py in 0..size {
            for px in 0..size {
                let src_x = view_start_x + (px as f32 * view_size / size as f32);
                let src_y = view_start_y + (py as f32 * view_size / size as f32);

                if src_x >= 0.0 && src_x < size as f32 && src_y >= 0.0 && src_y < size as f32 {
                    let pixel = full_image.get_pixel(src_x as u32, src_y as u32);
                    image.put_pixel(px, py, *pixel);
                }
            }
        }

        image
    }

    fn draw_cantor_square(
        &self,
        image: &mut RgbaImage,
        x: i32,
        y: i32,
        size: u32,
        iterations: u32,
        color: Rgba<u8>,
    ) {
        if size == 0 {
            return;
        }

        if iterations == 0 {
            self.draw_filled_rectangle(image, x, y, size, color);
            return;
        }

        let sub_size = size / 3;

        for i in [0, 2].iter() {
            for j in [0, 2].iter() {
                self.draw_cantor_square(
                    image,
                    x + (i * sub_size) as i32,
                    y + (j * sub_size) as i32,
                    sub_size,
                    iterations - 1,
                    color,
                );
            }
        }
    }

    fn draw_filled_rectangle(
        &self,
        image: &mut RgbaImage,
        x: i32,
        y: i32,
        size: u32,
        color: Rgba<u8>,
    ) {
        for i in 0..size {
            for j in 0..size {
                let px = x + i as i32;
                let py = y + j as i32;
                if px >= 0 && px < image.width() as i32 && py >= 0 && py < image.height() as i32 {
                    image.put_pixel(px as u32, py as u32, color);
                }
            }
        }
    }

    fn rgba_to_color_image(&self, img: &RgbaImage) -> egui::ColorImage {
        let size = [img.width() as usize, img.height() as usize];
        let pixels = img
            .pixels()
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();
        egui::ColorImage { size, pixels }
    }

    fn export_image(&self, filename: &str) {
        let size = 2187;
        let image = self.generate_cantor_image(size);
        image.save(filename).expect("Erreur lors de la sauvegarde");
        println!("✓ Image sauvegardée: {}", filename);
    }

    fn export_svg(&self, filename: &str) {
        
        let size = 2187; // 3^7 pour être divisible
        let color = self.colors[self.selected_color];
        
        let mut document = Document::new()
            .set("width", size)
            .set("height", size)
            .set("viewBox", (0, 0, size, size));
        
        // Dessiner le fond blanc
        let background = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", size)
            .set("height", size)
            .set("fill", "white");
        document = document.add(background);
        
        // Générer les rectangles de la fractale
        let rectangles = self.generate_svg_rectangles(0, 0, size, self.iterations, color);
        for rect in rectangles {
            document = document.add(rect);
        }
        
        svg::save(filename, &document).expect("Erreur lors de la sauvegarde SVG");
        println!("✓ SVG sauvegardé: {}", filename);
    }

    fn generate_svg_rectangles(
        &self,
        x: u32,
        y: u32,
        size: u32,
        iterations: u32,
        color: image::Rgba<u8>,
    ) -> Vec<svg::node::element::Rectangle> {
        use svg::node::element::Rectangle;
        
        let mut rectangles = Vec::new();
        
        if size == 0 {
            return rectangles;
        }
        
        if iterations == 0 {
            let rect = Rectangle::new()
                .set("x", x)
                .set("y", y)
                .set("width", size)
                .set("height", size)
                .set("fill", format!("rgb({},{},{})", color[0], color[1], color[2]));
            rectangles.push(rect);
            return rectangles;
        }
        
        let sub_size = size / 3;
        for i in [0, 2].iter() {
            for j in [0, 2].iter() {
                let mut sub_rects = self.generate_svg_rectangles(
                    x + (i * sub_size),
                    y + (j * sub_size),
                    sub_size,
                    iterations - 1,
                    color,
                );
                rectangles.append(&mut sub_rects);
            }
        }
        
        rectangles
    }
}
