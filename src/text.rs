use egui::Align2;
pub fn calculate_text_position_by_alignment(ui: &mut egui::Ui, p: &crate::Placeholder, text: &String) -> f32 {
    let font_id = egui::FontId::new(p.font_size, p.font_family.clone());
    let galley = ui.painter().layout_no_wrap(text.to_string(), font_id, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 255));
    let mut pos_x = 0.;
    match p.text_align {
        Align2::LEFT_CENTER => pos_x = p.rect.min.x,
        Align2::CENTER_CENTER => {
            let p_width = p.rect.max.x - p.rect.min.x;
            let center_anchor_x = p_width / 2.;

            let new_width = galley.size().x;
            let center_new_x = new_width / 2.;

            let offset = center_anchor_x - center_new_x;
            pos_x = p.rect.min.x + offset;
        }
        Align2::RIGHT_CENTER => {
            let p_width = p.rect.max.x - p.rect.min.x;

            let new_width = galley.size().x;

            let offset = p_width - new_width;
            pos_x = p.rect.min.x + offset;
        }
        _ => (),
    };
    pos_x
}

