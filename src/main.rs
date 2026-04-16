use eframe::egui;
use egui::Sense;
use std::{fs, time::SystemTime};

struct CertGen {
    name: String,
    level: String,
    boolean: bool,
    image_last_modified: SystemTime,
    placeholders: Vec<Placeholder>,
    focused_placeholder_idx: Option<usize>,
    scene_rect: egui::Rect,
}

impl Default for CertGen {
    fn default() -> Self {
        Self {
            name: String::default(),
            level: String::default(),
            boolean: bool::default(),
            image_last_modified: SystemTime::UNIX_EPOCH,
            placeholders: Vec::new(),
            focused_placeholder_idx: None,
            scene_rect: egui::Rect::ZERO,
        }
    }
}

impl CertGen {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            image_last_modified: SystemTime::UNIX_EPOCH,
            ..Self::default()
        }
    }
}

struct Placeholder {
    id: String,
    rect: egui::Rect,
    font_size: f32,
}

impl Placeholder {
    fn new(id: &str, font_size: f32) -> Self {
        Placeholder {
            id: id.to_string(),
            rect: egui::Rect::from_min_size(
                egui::Pos2 { x: 500., y: 500. },
                egui::Vec2 { x: 500., y: 500. },
            ),
            font_size,
        }
    }
}

impl eframe::App for CertGen {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        egui::Panel::top("top panel").show_inside(ui, |ui| {
            ui.heading("HanTalk Certificate Maker");
            ui.separator();
            ui.add(
                egui::TextEdit::singleline(&mut self.name)
                    .hint_text("Write the name of the student here"),
            );
            ui.add(
                egui::TextEdit::singleline(&mut self.level)
                    .hint_text("Write the level of the student here"),
            );
            if ui.button("Generate Certificate!").clicked() {
                self.boolean = !self.boolean;
            }
            ui.end_row();
        });

        egui::Panel::left("side_panel").show_inside(ui, |ui| {
            ui.heading("Control Panel");

            ui.collapsing("Placeholders", |ui| {
                for (idx, p) in self.placeholders.iter_mut().enumerate() {
                    let response = ui.text_edit_singleline(&mut p.id);
                    if response.gained_focus() {
                        self.focused_placeholder_idx = Some(idx);
                    }
                }
            });

            if ui.button("+ Add a new placeholder").clicked() {
                self.placeholders
                    .push(Placeholder::new("New Placeholder", 16.));
            }

            ui.separator();
            if let Some(idx) = self.focused_placeholder_idx {
                ui.add(egui::Slider::new(&mut self.placeholders[idx].font_size, 1.0..=100.0).text("Font Size"));
                if ui.button("+").clicked() {
                    self.placeholders[idx].font_size += 1.;
                }
                if ui.button("-").clicked() {
                    self.placeholders[idx].font_size -= 1.;
                }
            }
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // TODO: make it so that there's a dialog that asks whether or not we want to load the
            // new file instead
            let metadata = fs::metadata("Welcome_Certificate_new.jpg").unwrap();
            if let Ok(time) = metadata.modified() {
                if self.image_last_modified == SystemTime::UNIX_EPOCH {
                    self.image_last_modified = time;
                } else if self.image_last_modified != time {
                    ctx.forget_image("bytes://../Welcome_Certificate_new.jpg");
                    self.image_last_modified = time;
                    println!("image forgotten!: ${time:?}");
                }
            }
            // ui.image(egui::include_image!("../Welcome_Certificate_new.jpg"));
            // let response = ui.allocate_response(
            //     ui.available_size_before_wrap(),
            //     egui::Sense::click_and_drag(),
            // );
            // if response.clicked() {
            //     if let Some(click_pos) = response.interact_pointer_pos() {
            //         // Find the selected placeholder
            //         let mut clicked_a_placeholder = false;
            //         for (idx, p) in self.placeholders.iter().enumerate() {
            //             if p.rect.contains(click_pos) {
            //                 self.focused_placeholder_idx = Some(idx);
            //                 clicked_a_placeholder = true;
            //             }
            //         }
            //         if !clicked_a_placeholder {
            //             self.focused_placeholder_idx = None;
            //         }
            //     }
            // } else if response.dragged()
            //     && let Some(p) = self.focused_placeholder_idx
            // {
            //     let prev_rect = self.placeholders[p].rect;
            //     self.placeholders[p].rect = prev_rect.translate(response.drag_delta());
            // }
            //
            // if ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
            //     self.focused_placeholder_idx = None;
            // }

            let scene = egui::Scene::new()
                .max_inner_size(ui.available_size_before_wrap())
                .zoom_range(0.1..=2.0);

            ui.label(format!("Scene rect: {}", self.scene_rect));

            let mut is_any_placeholder_clicked = false;

            let scene_res = scene.show(ui, &mut self.scene_rect, |ui| {
                ui.image(egui::include_image!("../Welcome_Certificate_new.jpg"));
                let mut font_id = egui::FontId::default();
                // Draw rect on focused placeholder
                if let Some(p_idx) = &self.focused_placeholder_idx {
                    ui.painter().rect_stroke(
                        self.placeholders[*p_idx].rect,
                        0.,
                        egui::Stroke::new(2., egui::Color32::BLUE),
                        egui::StrokeKind::Outside,
                    );
                }
                // Draw each placeholder's text
                for (idx, p) in self.placeholders.iter_mut().enumerate() {
                    font_id.size = p.font_size;
                    p.rect = ui.painter().text(
                        p.rect.min,
                        egui::Align2::LEFT_TOP,
                        &p.id,
                        font_id.clone(),
                        egui::Color32::BLACK,
                    );
                    let p_res = ui.interact(p.rect, egui::Id::new(idx), Sense::click_and_drag());
                    if p_res.clicked() {
                        self.focused_placeholder_idx = Some(idx);
                        is_any_placeholder_clicked = true;
                    }
                    // Hover gives a highlight on the hovered item
                    if p_res.hovered() {
                        ui.painter().rect_filled(
                            p.rect,
                            0.,
                            egui::Color32::from_rgba_unmultiplied(0, 0, 255, 100),
                        );
                    }
                    if let Some(focus_idx) = self.focused_placeholder_idx
                        && idx == focus_idx
                        && p_res.dragged()
                    {
                        p.rect = p.rect.translate(p_res.drag_delta())
                    }
                    if ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.focused_placeholder_idx = None;
                    }
                }
            });

            if scene_res.response.clicked() && !is_any_placeholder_clicked {
                self.focused_placeholder_idx = None;
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    // let mut img = ImageReader::open("Welcome_Certificate.jpg").unwrap().decode().unwrap();
    //
    // let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/OTF/CodeNewRomanNerdFont-Regular.otf")).unwrap();
    //
    // let width = img.width();
    // let height = img.height();
    // drawing::draw_text_mut(&mut img, Rgba::from([120,120,120,255]), width as i32/2, height as i32/2, 200., &font, "Anjay");
    // let _ = img.save("Welcome_Certificate_new.jpg");

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "HanTalk Certificate Maker",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_debug_on_hover(false);
            Ok(Box::new(CertGen::new(cc)))
        }),
    )
}
