use std::time::Duration;

use crate::services::MusicService;
use crate::ui::{music_buttons::MusicButtons, music_path_entry_ui::MusicPathEntryUI};
use eframe::egui::{self, TextureHandle};
pub struct MusicPlayer {
    music_service: MusicService,
    music_path_entry_ui: MusicPathEntryUI,
    music_button_ui: MusicButtons,
    cover_texture: Option<TextureHandle>,
    pos: u64,
    total_duration: Option<u64>,
    // timestamp_text: String,
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
            // timestamp_text: String,
        }
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

            if self.music_service.is_music_loaded() {
                ui.label(format!(
                    "song name: {}",
                    self.music_service.music_file.name()
                ));
            
                if self.total_duration.is_none() {
                    self.total_duration =  Some(self.music_service.get_total_duration().unwrap().as_secs());
                }

                let response = ui.add(egui::Slider::new(&mut self.pos, 0..=self.music_service.get_total_duration().unwrap().as_secs()));

                if response.dragged() {
                    self.music_service.set_pos(Duration::from_secs(self.pos));
                } else {
                    self.pos = self.music_service.get_pos().unwrap().as_secs();
                }
                ui.horizontal(|ui| {

                    if self.music_button_ui.show_play_button(ui).clicked() {
                        self.music_service.resume();
                    }

                    if self.music_button_ui.show_pause_button(ui).clicked() {
                        self.music_service.pause();
                    }

                    if self.music_button_ui.show_stop_button(ui).clicked() {
                        self.music_service.stop();
                    }
                });
                
            }
        });
    }
}
