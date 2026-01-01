use eframe::egui::{
    self, Color32, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Vec2, vec2,
};

use crate::utils::color_util;

pub struct MusicButtons {
    desired_size: Vec2,
}

impl MusicButtons {
    const BUTTON_BG_IDLE: Color32 = Color32::from_rgb(30, 30, 35);
    const BUTTON_BG_HOVER: Color32 = Color32::from_rgb(45, 45, 50);

    pub fn new() -> Self {
        Self {
            desired_size: vec2(60.0, 25.0),
        }
    }

    // --- PLAY BUTTON ---
    pub fn show_play_button(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(self.desired_size, Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let how_hovered = ui.ctx().animate_bool(response.id, response.hovered());

            self.handle_button_background(ui, &response, painter, &rect, how_hovered);

            let play_icon_points = Self::calculate_play_icon_points(painter, &rect);
            let icon_color = self.get_themed_icon_color(ui, &response, how_hovered);

            painter.add(Shape::convex_polygon(
                play_icon_points,
                icon_color,
                Stroke::NONE,
            ));
        }
        self.apply_cursor(&response, ui);
        response
    }

    // --- PAUSE BUTTON ---
    pub fn show_pause_button(&self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(self.desired_size, Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let how_hovered = ui.ctx().animate_bool(response.id, response.hovered());

            self.handle_button_background(ui, &response, painter, &rect, how_hovered);

            let (left_bar, right_bar) = Self::calculate_pause_bar_rects(painter, &rect);
            let icon_color = self.get_themed_icon_color(ui, &response, how_hovered);

            painter.rect_filled(left_bar, 1.0, icon_color);
            painter.rect_filled(right_bar, 1.0, icon_color);
        }
        self.apply_cursor(&response, ui);
        response
    }

    // --- STOP BUTTON ---
    pub fn show_stop_button(&self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(self.desired_size, Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let how_hovered = ui.ctx().animate_bool(response.id, response.hovered());

            self.handle_button_background(ui, &response, painter, &rect, how_hovered);

            let icon_rect = Self::calculate_stop_icon_rect(painter, &rect);
            let icon_color = self.get_themed_icon_color(ui, &response, how_hovered);

            painter.rect_filled(icon_rect, 1.0, icon_color);
        }
        self.apply_cursor(&response, ui);
        response
    }

    // --- TIMELINE SLIDER ---
    pub fn timeline_slider_with_time(
        &self,
        ui: &mut egui::Ui,
        current_sec: &mut u64,
        total_sec: u64,
    ) -> egui::Response {
        ui.horizontal(|ui| {
            let text_color = ui.visuals().weak_text_color();

            let current_time_str = self.format_time(*current_sec);

            let total_time_str = self.format_time(total_sec);
            let label_style = |text: String| {
                egui::RichText::new(text)
                    .monospace()
                    .size(12.0)
                    .color(text_color)
            };

            ui.add(egui::Label::new(label_style(current_time_str)));

            let slider_res = self.timeline_slider(ui, current_sec, total_sec);

            ui.add(egui::Label::new(label_style(total_time_str)));

            slider_res
        })
        .inner
    }

    pub fn timeline_slider(
        &self,
        ui: &mut egui::Ui,
        current_sec: &mut u64,
        total_sec: u64,
    ) -> egui::Response {
        let desired_size = egui::vec2(ui.available_width(), 16.0);
        let (rect, mut response) = ui.allocate_at_least(desired_size, Sense::click_and_drag());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let is_hovered = response.hovered() || response.dragged();
            let how_hovered = ui.ctx().animate_bool(response.id, is_hovered);

            let thickness = egui::lerp(2.0..=4.0, how_hovered);

            if let Some(mouse_pos) = response.interact_pointer_pos() {
                let percentage = ((mouse_pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
                *current_sec = (percentage * total_sec as f32) as u64;
                response.mark_changed();
            }

            let progress = if total_sec > 0 {
                (*current_sec as f32 / total_sec as f32).clamp(0.0, 1.0)
            } else {
                0.0
            };

            painter.rect_filled(
                Rect::from_center_size(rect.center(), vec2(rect.width(), thickness)),
                thickness / 2.0,
                ui.visuals().extreme_bg_color,
            );

            let progress_width = progress * rect.width();
            let progress_rect = Rect::from_min_size(
                egui::pos2(rect.min.x, rect.center().y - thickness / 2.0),
                vec2(progress_width, thickness),
            );
            painter.rect_filled(
                progress_rect,
                thickness / 2.0,
                ui.visuals().selection.bg_fill,
            );

            if how_hovered > 0.0 {
                let handle_radius = egui::lerp(0.0..=6.0, how_hovered);
                let handle_pos = egui::pos2(rect.min.x + progress_width, rect.center().y);
                painter.circle_filled(handle_pos, handle_radius, Color32::WHITE);
            }
        }
        self.apply_cursor(&response, ui);
        response
    }

    // --- SHARED HELPERS ---

    fn handle_button_background(
        &self,
        ui: &egui::Ui,
        response: &Response,
        painter: &Painter,
        rect: &Rect,
        how_hovered: f32,
    ) {
        let bg_color = if response.is_pointer_button_down_on() {
            ui.visuals().widgets.active.bg_fill
        } else {
            color_util::lerp_color(Self::BUTTON_BG_IDLE, Self::BUTTON_BG_HOVER, how_hovered)
        };

        painter.rect_filled(*rect, egui::CornerRadius::same(4), bg_color);

        if how_hovered > 0.0 {
            let stroke_color = ui
                .visuals()
                .selection
                .bg_fill
                .linear_multiply(how_hovered * 0.5);

            painter.rect_stroke(
                *rect,
                egui::CornerRadius::same(4),
                egui::Stroke::new(1.0, stroke_color),
                egui::StrokeKind::Inside,
            );
        }
    }
    fn get_themed_icon_color(
        &self,
        ui: &egui::Ui,
        response: &Response,
        how_hovered: f32,
    ) -> Color32 {
        if response.is_pointer_button_down_on() {
            ui.visuals().selection.bg_fill
        } else {
            color_util::lerp_color(
                ui.visuals().text_color(),
                ui.visuals().selection.bg_fill,
                how_hovered,
            )
        }
    }

    fn apply_cursor(&self, response: &Response, ui: &egui::Ui) {
        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }

    fn format_time(&self, seconds: u64) -> String {
        let h = seconds / 3600;
        let m = (seconds % 3600) / 60;
        let s = seconds % 60;
        if h > 0 {
            format!("{}:{:02}:{:02}", h, m, s)
        } else {
            format!("{}:{:02}", m, s)
        }
    }

    // --- GEOMETRY CALCULATION WITH PIXEL SNAPPING ---

    fn calculate_play_icon_points(painter: &Painter, rect: &Rect) -> Vec<Pos2> {
        let icon_size = 10.0;
        let cx = painter.round_to_pixel_center(rect.center().x) + 1.0;
        let cy = painter.round_to_pixel_center(rect.center().y);
        let h = icon_size / 2.0;
        vec![
            Pos2::new(cx - h, cy - h),
            Pos2::new(cx - h, cy + h),
            Pos2::new(cx + h, cy),
        ]
    }

    fn calculate_pause_bar_rects(painter: &Painter, rect: &Rect) -> (Rect, Rect) {
        let h = 10.0;
        let w = 3.0;
        let g = 3.0;
        let cx = painter.round_to_pixel_center(rect.center().x);
        let cy = painter.round_to_pixel_center(rect.center().y);
        let ty = cy - (h / 2.0);
        (
            Rect::from_min_size(Pos2::new(cx - (g / 2.0) - w, ty), vec2(w, h)),
            Rect::from_min_size(Pos2::new(cx + (g / 2.0), ty), vec2(w, h)),
        )
    }

    fn calculate_stop_icon_rect(painter: &Painter, rect: &Rect) -> Rect {
        let size = 10.0;
        let cx = painter.round_to_pixel_center(rect.center().x);
        let cy = painter.round_to_pixel_center(rect.center().y);
        Rect::from_center_size(Pos2::new(cx, cy), vec2(size, size))
    }
}
