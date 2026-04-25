use egui::Align2;
pub fn calculate_text_position_by_alignment(
    p: &crate::Placeholder,
    scale: &ab_glyph::PxScale,
    font: &ab_glyph::FontVec,
    text: &str,
) -> f32 {
    let size = imageproc::drawing::text_size(*scale, font, text);
    let mut pos_x = 0.;
    match p.text_align {
        Align2::LEFT_CENTER => pos_x = p.rect.min.x,
        Align2::CENTER_CENTER => {
            let p_width = p.rect.max.x - p.rect.min.x;
            let center_anchor_x = p_width / 2.;

            let new_width = size.0;
            let center_new_x = new_width as f32 / 2.;

            let offset = center_anchor_x - center_new_x;
            pos_x = p.rect.min.x + offset;
        }
        Align2::RIGHT_CENTER => {
            let p_width = p.rect.max.x - p.rect.min.x;

            let new_width = size.0;

            let offset = p_width - new_width as f32;
            pos_x = p.rect.min.x + offset;
        }
        _ => (),
    };
    pos_x
}

pub fn text_align_to_str(align: Align2) -> String {
    match align {
        egui::Align2::LEFT_CENTER => "󰉢 Align Left".to_string(),
        egui::Align2::CENTER_CENTER => " Justify (Center)".to_string(),
        egui::Align2::RIGHT_CENTER => "󰉣 Align Right".to_string(),
        _ => "".to_string(),
    }
}
