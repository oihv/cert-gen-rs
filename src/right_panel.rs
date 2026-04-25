impl crate::CertGen {
    pub fn right_panel(&mut self, ui: &mut egui::Ui) {
        if ui.button("X").clicked() {
            self.right_panel_expand = false;
        }
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
    }
}
