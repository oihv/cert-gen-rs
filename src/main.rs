use eframe::egui;
use egui::{FontFamily, Sense, Widget};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{default, thread};
use std::{fs, time::SystemTime};

mod font;
mod parser;
use crate::font::{install_default_font, install_new_font};
mod generate;
mod source;
mod text;
use crate::source::Source;

struct CertGenImg {
    path: Option<String>,
    last_modified: SystemTime,
    size: egui::Vec2,
}

impl Default for CertGenImg {
    fn default() -> Self {
        Self {
            path: None,
            last_modified: SystemTime::UNIX_EPOCH,
            size: egui::Vec2::ZERO,
        }
    }
}

struct CertGen {
    left_panel_expand: bool,
    image: CertGenImg,
    source: Source,
    placeholders: Vec<Placeholder>,
    focused_placeholder_idx: Option<usize>,
    scene_rect: egui::Rect,
    available_fonts: Vec<egui::FontFamily>,
    font_vec_handles: HashMap<String, Vec<u8>>,
    // TODO: move to another struct too, just like source
    generate: generate::CertGenGenerate,
    generate_progress: Option<Arc<Mutex<f32>>>,
}

impl Default for CertGen {
    fn default() -> Self {
        Self {
            left_panel_expand: true,
            image: CertGenImg::default(),
            source: Source::default(),
            placeholders: Vec::new(),
            focused_placeholder_idx: None,
            scene_rect: egui::Rect::ZERO,
            available_fonts: vec![FontFamily::Proportional],
            font_vec_handles: HashMap::new(),
            generate: generate::CertGenGenerate::default(),
            generate_progress: None,
        }
    }
}

impl CertGen {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut ret = Self { ..Self::default() };
        ret.font_vec_handles.insert(
            format!("{}", FontFamily::Proportional),
            // TODO: how to synchronize egui context default font installation and this?
            include_bytes!("../CaskaydiaCoveNerdFont-Light.ttf").to_vec(),
        );
        ret
    }
}

#[derive(Clone)]
struct Placeholder {
    id: String,
    rect: egui::Rect,
    pos: egui::Pos2,
    color: egui::Color32,
    font_size: f32,
    font_family: egui::FontFamily,
    text_align: egui::Align2,
    screen_align: Option<TextImageAlign>,
}

#[derive(Clone)]
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
            color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 255),
            font_size,
            font_family: egui::FontFamily::Proportional,
            text_align: egui::Align2::LEFT_CENTER,
            screen_align: None,
        }
    }
}

impl eframe::App for CertGen {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        egui::Panel::top("top panel").show_inside(ui, |ui| {
            ui.heading("CertGen - Certificate Generator");
            if !self.left_panel_expand && ui.button("Control Panel").clicked() {
                self.left_panel_expand = true;
            }
        });

        // TODO: make it collapsible too like the left panel
        egui::Panel::right("right_panel")
            .min_size(ui.available_size().x * 0.15)
            .show_animated_inside(ui, true, |ui| {
                ui.heading("Source Data Viewer");
                ui.separator();

                if ui.button("Select Source…").clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_file()
                {
                    self.source.path = Some(path.display().to_string());
                    // TODO! reload the data if the file has been modified.
                    self.source.load_data(path);
                }

                if let Some(path) = &self.source.path {
                    ui.label(format!("Selected source: {path}"));
                    ui.separator();
                    egui::ScrollArea::both()
                        .max_height(ui.available_height() * 0.25)
                        .show(ui, |ui| {
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                            ui.set_max_height(200.);
                            // TODO: add a table that shows the values of the source, make it scrollable.
                            egui::Grid::new("source_grid")
                                .num_columns(self.source.data[0].len())
                                // .spacing([40.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    for header in &self.source.header {
                                        // TODO: how to center this?
                                        ui.label(
                                            egui::RichText::new(header)
                                                .color(egui::Color32::BLACK)
                                                .background_color(egui::Color32::LIGHT_GRAY),
                                        );
                                    }
                                    ui.end_row();
                                    for row in &self.source.data {
                                        for val in row {
                                            ui.label(val.to_string());
                                        }
                                        ui.end_row();
                                    }
                                })
                        });
                } else {
                    ui.label("Selected source: Not Selected");
                }

                ui.separator();

                ui.label("File name template");
                ui.text_edit_singleline(&mut self.generate.template);

                ui.separator();
                if ui.button("Select output directory…").clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_folder()
                {
                    self.generate.dir = Some(path.display().to_string());
                }
                if let Some(path) = &self.generate.dir {
                    ui.label(format!("Selected directory: {path}"));
                } else {
                    ui.label(
                        "Selected directory: Not Selected (Default to same directory as image)",
                    );
                }
            });

        egui::Panel::left("left_panel")
            .min_size(ui.available_size().x * 0.25)
            .show_animated_inside(ui, self.left_panel_expand, |ui| {
                if ui.button("Collapse").clicked() {
                    self.left_panel_expand = false;
                }
                ui.heading("Control Panel");
                if ui.button("Select Template…").clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_file()
                {
                    self.image.path = Some(path.display().to_string());
                }
                if let Some(path) = &self.image.path {
                    ui.label(format!("Selected image: {path}"));
                } else {
                    ui.label("Selected image: Not Selected");
                }
                ui.separator();

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
                    let p = &mut self.placeholders[idx];
                    egui::Grid::new("my_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Font Size");
                            ui.horizontal(|ui| {
                                ui.add(egui::Slider::new(&mut p.font_size, 1.0..=500.0));
                                ui.horizontal(|ui| {
                                    if ui.button("+").clicked() {
                                        p.font_size += 1.;
                                    }
                                    if ui.button("-").clicked() {
                                        p.font_size -= 1.;
                                    }
                                });
                            });

                            ui.end_row();

                            ui.label("Color");
                            ui.color_edit_button_srgba(&mut p.color);

                            ui.end_row();

                            ui.label("Alignment");
                            egui::ComboBox::from_id_salt("Text alignment")
                                .selected_text(text::text_align_to_str(p.text_align))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut p.text_align,
                                        egui::Align2::LEFT_CENTER,
                                        "󰉢 Align Left",
                                    );
                                    ui.selectable_value(
                                        &mut p.text_align,
                                        egui::Align2::CENTER_CENTER,
                                        " Justify (Center)",
                                    );
                                    ui.selectable_value(
                                        &mut p.text_align,
                                        egui::Align2::RIGHT_CENTER,
                                        "󰉣 Align Right",
                                    );
                                });

                            ui.end_row();
                            ui.label("Position");
                            ui.horizontal(|ui| {
                                if ui.button("󰘞").clicked() {
                                    p.screen_align = Some(TextImageAlign::Horizontal);
                                }
                                if ui.button("󰘢").clicked() {
                                    p.screen_align = Some(TextImageAlign::Vertical);
                                }
                            });
                            ui.end_row();
                            ui.label("Font");
                            egui::ComboBox::from_label("Font")
                                .selected_text(format!("{}", p.font_family))
                                .show_ui(ui, |ui| {
                                    for font in self.available_fonts.clone() {
                                        ui.selectable_value(
                                            &mut p.font_family,
                                            font.clone(),
                                            format!("{}", font),
                                        );
                                    }
                                });
                            ui.end_row();
                        });

                    if ui.button("Delete").clicked() {
                        self.focused_placeholder_idx = None;
                        self.placeholders.remove(idx);
                    }
                    ui.separator();
                }

                if ui.button("Install Local Font…").clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_file()
                {
                    // egui::Context all points to the same context even when cloned (from docs)
                    install_new_font(self, &mut ui.ctx().clone(), path);
                }

                ui.separator();

                if ui.button("Generate").clicked()
                    && let Some(path) = &self.image.path
                {
                    self.generate_progress = Some(Arc::new(Mutex::new(0.)));
                    // Clones of CertGen fields to be moved inside the closure
                    let generate_progress = self.generate_progress.as_ref().unwrap().clone();
                    let generate = self.generate.clone();
                    let placeholders = self.placeholders.clone();
                    let path = path.clone();
                    let font_vec_handles = self.font_vec_handles.clone();
                    let source = self.source.clone();
                    let img_src = image::ImageReader::open(path).unwrap().decode().unwrap();
                    let ctx = ui.ctx().clone();

                    thread::spawn(move || {
                        generate::generate_certificates(
                            generate_progress,
                            source,
                            placeholders,
                            font_vec_handles,
                            img_src,
                            ctx,
                            generate,
                        );
                    });
                }
                if let Some(prog) = &self.generate_progress {
                    let mut prog_copy = 0.;
                    if let Ok(prog) = prog.try_lock() {
                        prog_copy = *prog;
                        ui.add(egui::widgets::ProgressBar::new(prog_copy).show_percentage());
                    }
                    if prog_copy == 1. {
                        self.generate_progress = None;
                    }
                    dbg!(prog_copy);
                }
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // TODO: make it so that there's a dialog that asks whether or not we want to load the
            // new file instead
            if let Some(path) = &self.image.path {
                let metadata = fs::metadata(path).unwrap();
                if let Ok(time) = metadata.modified() {
                    if self.image.last_modified == SystemTime::UNIX_EPOCH {
                        self.image.last_modified = time;
                    } else if self.image.last_modified != time {
                        ctx.forget_image(&format!("file://{path}"));
                        self.image.last_modified = time;
                        println!("image forgotten!: ${time:?}");
                    }
                }

                let scene = egui::Scene::new()
                    .max_inner_size(ui.available_size_before_wrap())
                    .zoom_range(0.1..=2.0);

                let mut is_any_placeholder_clicked = false;

                let scene_res = scene.show(ui, &mut self.scene_rect, |ui| {
                    // let img_res = ui.image(format!("file://{path}"));
                    let img_res = egui::Image::new(format!("file://{path}"))
                        .fit_to_original_size(1.)
                        .ui(ui);
                    ui.label(format!("img_rect: {}", img_res.rect));

                    if self.image.size.x != img_res.rect.max.x - img_res.rect.min.x
                        || self.image.size.y != img_res.rect.max.y - img_res.rect.min.y
                    {
                        self.image.size = egui::Vec2::new(
                            img_res.rect.max.x - img_res.rect.min.x,
                            img_res.rect.max.y - img_res.rect.min.y,
                        );
                    }

                    // Draw each placeholder's text
                    for (idx, p) in self.placeholders.iter_mut().enumerate() {
                        // Handle screen align
                        if let Some(align) = &p.screen_align {
                            match align {
                                TextImageAlign::Horizontal => {
                                    let img_width = img_res.rect.max.x - img_res.rect.min.x;
                                    let p_width = p.rect.max.x - p.rect.min.x;
                                    match p.text_align {
                                        egui::Align2::LEFT_CENTER => {
                                            p.pos.x = img_width / 2. - p_width / 2.
                                        }
                                        egui::Align2::CENTER_CENTER => p.pos.x = img_width / 2.,
                                        egui::Align2::RIGHT_CENTER => {
                                            p.pos.x = img_width / 2. + p_width / 2.
                                        }
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
                        p.rect =
                            ui.painter()
                                .text(p.pos, p.text_align, &p.id, font_id.clone(), p.color);
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
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "CertGen",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_debug_on_hover(false);
            install_default_font(&mut cc.egui_ctx.clone());
            Ok(Box::new(CertGen::new(cc)))
        }),
    )
}
