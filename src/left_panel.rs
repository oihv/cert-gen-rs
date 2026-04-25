use crate::*;
impl crate::CertGen {

    pub fn left_panel(&mut self, ui: &mut egui::Ui) {
        if ui.button("X").clicked() {
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
    }
}
