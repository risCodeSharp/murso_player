use eframe::egui::{
    self, Color32, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Vec2, vec2,
};

use crate::utils::color_util;

pub struct MusicButtons {
    desired_size: Vec2,
}

impl MusicButtons {
    // Constants for consistent styling
    const BUTTON_COLOR: Color32 = Color32::from_rgb(35, 39, 48);
    const BUTTON_WIDGET_COLOR: Color32 = Color32::from_rgb(0, 122, 255);
    const HOVERED_BUTTON_WIDGET_COLOR: Color32 = Color32::from_rgb(60, 160, 255);
    const HOVER_BORDER_COLOR: Color32 = Color32::from_rgb(0, 122, 255);

    pub fn new() -> Self {
        Self {
            desired_size: vec2(60.0, 25.0),
        }
    }

    pub fn show_play_button(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(self.desired_size, Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // --- Background ---

            let how_hovered = ui
                .ctx()
                .animate_bool_with_time(response.id, response.hovered(), 0.1);
            self.handle_button_background(&response, painter, &rect, how_hovered);

            // --- Play Icon (Triangle) ---
            let play_icon_points = Self::calculate_play_icon_points(painter, &rect);

            let icon_color = Self::get_animated_icon_color(&response,how_hovered);
            painter.add(Shape::convex_polygon(
                play_icon_points,
                icon_color,
                Stroke::NONE,
            ));
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        response
    }

    pub fn show_pause_button(&self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(self.desired_size, Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // --- Background ---
            let how_hovered = ui
                .ctx()
                .animate_bool_with_time(response.id, response.hovered(), 0.1);
            self.handle_button_background( &response, painter, &rect, how_hovered);

            // --- Pause Icon (Two Bars) ---
            let (left_bar, right_bar) = Self::calculate_pause_bar_rects(&painter, &rect);
            let icon_color = Self::get_animated_icon_color(&response,how_hovered);

            painter.rect_filled(left_bar, 1.0, icon_color);
            painter.rect_filled(right_bar, 1.0, icon_color);
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        response
    }

    pub fn show_stop_button(&self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(self.desired_size, Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // --- Background ---
            let how_hovered = ui
                .ctx()
                .animate_bool_with_time(response.id, response.hovered(), 0.1);
            self.handle_button_background( &response, painter, &rect, how_hovered);

            // --- Stop Icon (Square) ---
            let icon_rect = Self::calculate_stop_icon_rect(painter, &rect);
            let icon_color = Self::get_animated_icon_color(&response,how_hovered);
            painter.rect_filled(icon_rect, 1.0, icon_color);
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        response
    }

    fn handle_button_background(
        &self,
        response: &Response,
        painter: &Painter,
        rect: &Rect,
        how_hovered: f32,
    ) {
        // --- Background ---
        let bg_color = if response.is_pointer_button_down_on() {
            Color32::from_rgb(15, 18, 24)
        } else {
            let hover_bg = Color32::from_rgb(50, 55, 65);
            color_util::lerp_color(MusicButtons::BUTTON_COLOR, hover_bg, how_hovered)
        };
        painter.rect_filled(*rect, 4.0, bg_color);

        if how_hovered > 0.0 {
            let stroke_color = Color32::from_rgba_unmultiplied(
                MusicButtons::HOVER_BORDER_COLOR.r(),
                MusicButtons::HOVER_BORDER_COLOR.g(),
                MusicButtons::HOVER_BORDER_COLOR.b(),
                (how_hovered * 100.0) as u8,
            );
            painter.rect_stroke(*rect, 4.0, (1.0, stroke_color), egui::StrokeKind::Inside);
        }
    }

fn get_animated_icon_color(response: &Response, how_hovered: f32) -> Color32 {
        if response.is_pointer_button_down_on() {
            // Flash a bit brighter when actually clicked
            Color32::from_rgb(100, 180, 255)
        } else {
            color_util::lerp_color(
                Self::BUTTON_WIDGET_COLOR,
                Self::HOVERED_BUTTON_WIDGET_COLOR,
                how_hovered,
            )
        }
    }
    fn calculate_stop_icon_rect(painter: &Painter, rect: &Rect) -> Rect {
        let icon_size = 10.0;
        let cx = painter.round_to_pixel_center(rect.center().x);
        let cy = painter.round_to_pixel_center(rect.center().y);
        Rect::from_center_size(Pos2::new(cx, cy), vec2(icon_size, icon_size))
    }

    fn calculate_play_icon_points(painter: &Painter, rect: &Rect) -> Vec<Pos2> {
        let icon_size = 10.0;
        let cx = painter.round_to_pixel_center(rect.center().x) + 1.0; // +1 for visual balance
        let cy = painter.round_to_pixel_center(rect.center().y);
        let half_side = icon_size / 2.0;

        vec![
            Pos2::new(cx - half_side, cy - half_side),
            Pos2::new(cx - half_side, cy + half_side),
            Pos2::new(cx + half_side, cy),
        ]
    }

    fn calculate_pause_bar_rects(painter: &Painter, rect: &Rect) -> (Rect, Rect) {
        let icon_height = 10.0;
        let bar_width = 3.0;
        let gap = 3.0;

        let cx = painter.round_to_pixel_center(rect.center().x);
        let cy = painter.round_to_pixel_center(rect.center().y);

        let top_y = cy - (icon_height / 2.0);
        let left_bar_x = cx - (gap / 2.0) - bar_width;
        let right_bar_x = cx + (gap / 2.0);
        (
            Rect::from_min_size(Pos2::new(left_bar_x, top_y), vec2(bar_width, icon_height)),
            Rect::from_min_size(Pos2::new(right_bar_x, top_y), vec2(bar_width, icon_height)),
        )
    }
}
