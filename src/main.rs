use eframe::egui;
use egui::{FontFamily, Sense, Widget};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs, time::SystemTime};

mod left_panel;
mod right_panel;
mod central_panel;
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
    right_panel_expand: bool,
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
            right_panel_expand: true,
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
            if !self.right_panel_expand && ui.button("Show Right Panel").clicked() {
                self.right_panel_expand = true;
            }
        });

        // TODO: make it collapsible too like the left panel
        egui::Panel::right("right_panel")
            .min_size(ui.available_size().x * 0.15)
            .show_animated_inside(ui, self.right_panel_expand, |ui| self.right_panel(ui));

        egui::Panel::left("left_panel")
            .min_size(ui.available_size().x * 0.25)
            .show_animated_inside(ui, self.left_panel_expand, |ui| self.left_panel(ui));

        egui::CentralPanel::default().show_inside(ui, |ui| self.central_panel(ctx, ui));
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
