use crate::*;
impl crate::CertGen {
    pub fn central_panel(&mut self, ctx: egui::Context, ui: &mut egui::Ui) {
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
                    let p_res = ui.interact(p.rect, egui::Id::new(idx), Sense::click_and_drag());
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
    }
}
