use super::CertGen;
pub fn install_new_font(app_data: &mut CertGen, ui: &mut egui::Ui, path: std::path::PathBuf) {
    match std::fs::read(&path) {
        Ok(font_bytes) => {
            let mut fonts = ui.ctx().fonts(|f| f.definitions().clone());

            let mut path_clone = path.clone();
            path_clone.set_extension("");
            let font_name = path_clone
                .file_name()
                .expect("Font file doesn't have a name.")
                .to_str()
                .expect("Font name conversion to string failed.")
                .to_owned();

            fonts.font_data.insert(
                font_name.clone(),
                std::sync::Arc::new(egui::FontData::from_owned(font_bytes)),
            );
            fonts.families.insert(
                egui::FontFamily::Name(font_name.clone().into()),
                vec![font_name.clone()],
            );

            ui.ctx().set_fonts(fonts);
            app_data
                .available_fonts
                .push(egui::FontFamily::Name(font_name.clone().into()));
        }
        Err(err) => {
            eprintln!("Failed to read font file: {err}");
        }
    }
}
