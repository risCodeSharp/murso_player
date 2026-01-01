use crate::services::MusicService;
use crate::ui::{music_buttons::MusicButtons, music_path_entry_ui::MusicPathEntryUI};
use eframe::egui::{self, ScrollArea, TextureHandle};
pub struct MusicPlayer {
    music_service: MusicService,
    music_path_entry_ui: MusicPathEntryUI,
    music_button_ui: MusicButtons,
    cover_texture: Option<TextureHandle>,
    pos: u64,
    total_duration: Option<u64>,
    music_list: Vec<String>, // timestamp_text: String,
}

impl MusicPlayer {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            music_path_entry_ui: MusicPathEntryUI::new(),
            music_service: MusicService::new(),
            music_button_ui: MusicButtons::new(),
            cover_texture: None,
            pos: 0,
            total_duration: None,
            music_list: Vec::new(),
            // timestamp_text: String,
        }
    }

    pub fn display_music_list(&self, ui: &mut egui::Ui) {
        if !self.music_list.is_empty() {
            return;
        }

        ScrollArea::vertical().show(ui, |ui| {
            for item in &self.music_list {
                ui.label(item);
            }
            for item in &self.music_list {
                println!("{}", item);
            }
        });
    }
}

impl eframe::App for MusicPlayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.music_path_entry_ui.show(ui);

            self.music_path_entry_ui.on_submit(
                ctx,
                &mut self.music_service,
                &mut self.cover_texture,
            );

            if !self.music_service.is_music_loaded() {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.weak("Select a song to start the player...");
                });
            }
        });

        if self.music_service.is_music_loaded() {
            egui::TopBottomPanel::bottom("playback_bar")
                .resizable(false)
                .frame(
                    egui::Frame::NONE
                        .fill(ctx.style().visuals.panel_fill)
                        .inner_margin(10.0),
                )
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        // Display Song Name
                        ui.label(
                            egui::RichText::new(format!(
                                "🎵 {}",
                                self.music_service.music_file.name()
                            ))
                            .strong()
                            .color(ctx.style().visuals.widgets.active.fg_stroke.color),
                        );

                        // Timeline Logic
                        if self.total_duration.is_none() {
                            if let Some(duration) = self.music_service.get_total_duration() {
                                self.total_duration = Some(duration.as_secs());
                            }
                        }

                        let total = self.total_duration.unwrap_or(0);

                        // Capture slider response to handle dragging
                        let slider_res = self.music_button_ui.timeline_slider_with_time(
                            ui,
                            &mut self.pos,
                            total,
                        );

                        if slider_res.dragged() || slider_res.clicked() {
                            // Update the actual playback position while dragging
                            let _ = self
                                .music_service
                                .set_pos(std::time::Duration::from_secs(self.pos));
                        } else {
                            // Sync the slider position with the music service while NOT dragging
                            if let Some(current_pos) = self.music_service.get_pos() {
                                self.pos = current_pos.as_secs();
                            }
                        }


                        // Control Buttons 
                        ui.vertical_centered(|ui| {
                            ui.horizontal(|ui| {
                                let total_buttons_width =
                                    (60.0 * 3.0) + (ui.spacing().item_spacing.x * 2.0);
                                let padding = (ui.available_width() - total_buttons_width) / 2.0;

                                if padding > 0.0 {
                                    ui.add_space(padding);
                                }

                                if self.music_button_ui.show_pause_button(ui).clicked() {
                                    self.music_service.pause();
                                }
                                
                                if self.music_button_ui.show_play_button(ui).clicked() {
                                    self.music_service.resume();
                                }


                                if self.music_button_ui.show_stop_button(ui).clicked() {
                                    self.music_service.stop();
                                }
                            });
                        });
                    });
                });
        }
        ctx.request_repaint();
    }
}
