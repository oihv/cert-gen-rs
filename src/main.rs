use eframe::egui;
use egui::{FontFamily, Sense};
use std::{fs, time::SystemTime};

use crate::font::{install_default_font, install_new_font};
mod font;

struct CertGen {
    image_path: Option<String>,
    image_last_modified: SystemTime,
    placeholders: Vec<Placeholder>,
    focused_placeholder_idx: Option<usize>,
    scene_rect: egui::Rect,
    available_fonts: Vec<egui::FontFamily>,
    // font_vec_fonts: HashMap: <egui::FontFamily, ab_glyph::FontVec>,
}

impl Default for CertGen {
    fn default() -> Self {
        Self {
            image_path: None,
            image_last_modified: SystemTime::UNIX_EPOCH,
            placeholders: Vec::new(),
            focused_placeholder_idx: None,
            scene_rect: egui::Rect::ZERO,
            available_fonts: vec![FontFamily::Proportional],
        }
    }
}

impl CertGen {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { ..Self::default() }
    }

}

struct Placeholder {
    id: String,
    rect: egui::Rect,
    pos: egui::Pos2,
    font_size: f32,
    font_family: egui::FontFamily,
    text_align: egui::Align2,
    screen_align: Option<TextImageAlign>,
}

enum TextImageAlign {
    Horizontal,
    Vertical,
}

impl Placeholder {
    fn new(id: &str, font_size: f32) -> Self {
        Placeholder {
            id: id.to_string(),
            rect: egui::Rect::from_min_size(
                egui::Pos2 { x: 500., y: 500. },
                egui::Vec2 { x: 500., y: 500. },
            ),
            pos: egui::Pos2::new(50., 50.),
            font_size,
            font_family: egui::FontFamily::Proportional,
            text_align: egui::Align2::LEFT_TOP,
            screen_align: None,
        }
    }
}

impl eframe::App for CertGen {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        egui::Panel::top("top panel").show_inside(ui, |ui| {
            ui.heading("HanTalk Certificate Maker");
        });

        egui::Panel::left("side_panel").show_inside(ui, |ui| {
            ui.heading("Control Panel");
            if ui.button("Select Template…").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_file()
            {
                self.image_path = Some(path.display().to_string());
            }
            if let Some(path) = &self.image_path {
                ui.label(format!("Selected image: {path}"));
            } else {
                ui.label("Selected image: Not Selected");
            }

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
                ui.add(
                    egui::Slider::new(&mut self.placeholders[idx].font_size, 1.0..=100.0)
                        .text("Font Size"),
                );
                ui.horizontal(|ui| {
                    if ui.button("+").clicked() {
                        self.placeholders[idx].font_size += 1.;
                    }
                    if ui.button("-").clicked() {
                        self.placeholders[idx].font_size -= 1.;
                    }
                });

                ui.separator();

                // TODO: Change to selectable_value like screen alignment instead
                egui::ComboBox::from_label("Text alignment")
                    .selected_text(format!("{:?}", self.placeholders[idx].text_align))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.placeholders[idx].text_align,
                            egui::Align2::LEFT_CENTER,
                            "󰉢Align Left",
                        );
                        ui.selectable_value(
                            &mut self.placeholders[idx].text_align,
                            egui::Align2::CENTER_CENTER,
                            "Justify (Center)",
                        );
                        ui.selectable_value(
                            &mut self.placeholders[idx].text_align,
                            egui::Align2::RIGHT_CENTER,
                            "󰉣Align Right",
                        );
                    });

                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("󰘞").clicked() {
                        self.placeholders[idx].screen_align = Some(TextImageAlign::Horizontal);
                    }
                    if ui.button("󰘢").clicked() {
                        self.placeholders[idx].screen_align = Some(TextImageAlign::Vertical);
                    }
                });

                egui::ComboBox::from_label("Choose font")
                    .selected_text(format!("{}", self.placeholders[idx].font_family))
                    .show_ui(ui, |ui| {
                        for font in self.available_fonts.clone() {
                            ui.selectable_value(
                                &mut self.placeholders[idx].font_family,
                                font.clone(),
                                format!("{}", font),
                            );
                        }
                    });

                ui.separator();

                if ui.button("Delete").clicked() {
                    self.focused_placeholder_idx = None;
                    self.placeholders.remove(idx);
                }
            }
            ui.separator();

            if ui.button("Install Local Font…").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_file()
            {
                // egui::Context all points to the same context even when cloned (from docs)
                font::install_new_font(self, &mut ui.ctx().clone(), path);
            }

            ui.separator();

            if ui.button("Generate").clicked()
            {

            }
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // TODO: make it so that there's a dialog that asks whether or not we want to load the
            // new file instead
            if let Some(path) = &self.image_path {
                let metadata = fs::metadata(path).unwrap();
                if let Ok(time) = metadata.modified() {
                    if self.image_last_modified == SystemTime::UNIX_EPOCH {
                        self.image_last_modified = time;
                    } else if self.image_last_modified != time {
                        ctx.forget_image(&format!("bytes://{path}"));
                        self.image_last_modified = time;
                        println!("image forgotten!: ${time:?}");
                    }
                }

                let scene = egui::Scene::new()
                    .max_inner_size(ui.available_size_before_wrap())
                    .zoom_range(0.1..=2.0);

                let mut is_any_placeholder_clicked = false;

                let scene_res = scene.show(ui, &mut self.scene_rect, |ui| {
                    let img_res = ui.image(egui::include_image!("../Welcome_Certificate_new.jpg"));
                    ui.label(format!("img_rect: {}", img_res.rect));

                    // Draw each placeholder's text
                    for (idx, p) in self.placeholders.iter_mut().enumerate() {
                        // Handle screen align
                        if let Some(align) = &p.screen_align {
                            match align {
                                TextImageAlign::Horizontal => {
                                    let img_width = img_res.rect.max.x - img_res.rect.min.x;
                                    let p_width = p.rect.max.x - p.rect.min.x;
                                    match p.text_align {
                                        egui::Align2::LEFT_CENTER => p.pos.x = img_width / 2. - p_width / 2.,
                                        egui::Align2::CENTER_CENTER => p.pos.x = img_width / 2.,
                                        egui::Align2::RIGHT_CENTER => p.pos.x = img_width / 2. + p_width / 2.,
                                        _ => (),
                                    }
                                }
                                TextImageAlign::Vertical => {
                                    let img_height = img_res.rect.max.y - img_res.rect.min.y;
                                    let p_height = p.rect.max.y - p.rect.min.y;
                                    p.pos.y = img_height / 2. - p_height / 2.;
                                }
                            }
                            p.screen_align = None;
                        }

                        let font_id = egui::FontId::new(p.font_size, p.font_family.clone());
                        p.rect = ui.painter().text(
                            p.pos,
                            p.text_align,
                            &p.id,
                            font_id.clone(),
                            egui::Color32::BLACK,
                        );
                        // Get input with the placeholder
                        let p_res =
                            ui.interact(p.rect, egui::Id::new(idx), Sense::click_and_drag());
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
                        {
                            ui.painter().rect_stroke(
                                p.rect,
                                0.,
                                egui::Stroke::new(2., egui::Color32::BLUE),
                                egui::StrokeKind::Outside,
                            );
                            if p_res.dragged() {
                                p.pos += p_res.drag_delta();
                                p.rect = p.rect.translate(p_res.drag_delta());
                            }
                            ui.label(format!("focused: {:?}", p.rect));
                        }
                    }
                });
                // Get keyboard input
                if let Some(idx) = self.focused_placeholder_idx {
                    if ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.focused_placeholder_idx = None;
                    }
                    if ui.ctx().input(|i| i.key_pressed(egui::Key::Delete)) {
                        self.focused_placeholder_idx = None;
                        self.placeholders.remove(idx);
                    }
                }

                if scene_res.response.clicked() && !is_any_placeholder_clicked {
                    self.focused_placeholder_idx = None;
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("No image selected as template");
                });
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
            install_default_font(&mut cc.egui_ctx.clone());
            Ok(Box::new(CertGen::new(cc)))
        }),
    )
}
