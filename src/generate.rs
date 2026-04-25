use crate::parser;
use crate::{Placeholder, text};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct CertGenGenerate {
    pub progress: Option<Arc<Mutex<f32>>>,
    pub template: String,
    pub dir: Option<String>,
}


pub fn generate_certificates(
    generate_progress: Arc<Mutex<f32>>,
    source: crate::source::Source,
    placeholders: Vec<Placeholder>,
    font_vec_handles: HashMap<String, Vec<u8>>,
    img_src: image::DynamicImage,
    ctx: egui::Context,
    generate: crate::generate::CertGenGenerate
) {
    let total_work = source.data.len() * placeholders.len();
    let mut curr_work = 0;

    for (idx, row) in source.data.iter().enumerate() {
        let mut img = img_src.clone();
        for p in &placeholders {
            let font = ab_glyph::FontVec::try_from_vec(
                font_vec_handles
                    .get(&format!("{}", p.font_family))
                    .unwrap()
                    .to_vec(),
            )
            .unwrap();
            let text = crate::parser::construct_string(&p.id, &source.access_hash, row);
            // TODO: these values don't need to be computed multiple times
            // intended_text_height and scale should be computed once for all placeholders for each data
            // pos_x should be computed once for all placeholders and for all data
            let intended_text_height = p.rect.max.y - p.rect.min.y;
            let scale = ab_glyph::PxScale::from(
                intended_text_height / imageproc::drawing::text_size(1., &font, &p.id).1 as f32,
            );
            let pos_x = text::calculate_text_position_by_alignment(p, &scale, &font, &text);

            let color = image::Rgba::from([p.color.r(), p.color.g(), p.color.b(), p.color.a()]);
            imageproc::drawing::draw_text_mut(
                &mut img,
                color,
                pos_x as i32,
                p.rect.min.y as i32,
                scale,
                &font,
                &text,
            );
            curr_work += 1;
            *generate_progress.lock().unwrap() = curr_work as f32 / total_work as f32;
            ctx.request_repaint();
        }
        let mut dir = String::new();
        if let Some(ref dir_handle) = generate.dir {
            dir = dir_handle.to_string()
        }
        let _ = if generate.template.is_empty() {
            img.save(format!("{dir}Welcome_Certificate_new_{idx}.jpg"))
        } else {
            let file_name = parser::construct_string(&generate.template, &source.access_hash, row);
            img.save(format!("{dir}/{file_name}.jpg"))
        };
    }
}
