use super::CertGen;
pub fn install_new_font(app_data: &mut CertGen, ctx: &mut egui::Context, path: std::path::PathBuf) {
    match std::fs::read(&path) {
        Ok(font_bytes) => {
            let mut fonts = ctx.fonts(|f| f.definitions().clone());

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

            ctx.set_fonts(fonts);
            app_data
                .available_fonts
                .push(egui::FontFamily::Name(font_name.clone().into()));
        }
        Err(err) => {
            eprintln!("Failed to read font file: {err}");
        }
    }
}

pub fn install_default_font(ctx: &mut egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert("my_font".to_owned(),
       std::sync::Arc::new(
           // .ttf and .otf supported
           egui::FontData::from_static(include_bytes!("../CaskaydiaCoveNerdFont-Light.ttf"))
       )
    );

    // Put my font first (highest priority):
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);
}
