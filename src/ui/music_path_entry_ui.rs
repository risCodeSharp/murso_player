use crate::services::{MusicOpenError, MusicService};

use eframe::egui::{self, Response, TextureHandle, Ui};

pub struct MusicPathEntryUI {
    path: String,
    submit_response: Option<Response>,
}

impl MusicPathEntryUI {
    pub fn new() -> Self {
        Self {
            path: String::from("music/audio.mp3"),
            submit_response: None,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.path);
            let button = egui::Button::new("play");
            self.submit_response = Some(ui.add(button));
        });
    }

    pub fn on_submit(
        &mut self,
        _ctx: &egui::Context,
        service: &mut MusicService,
        _texture_handle: &mut Option<TextureHandle>,
    ) {
        if let Some(response) = &self.submit_response {
            if response.clicked() {
                match service.open(self.path.as_str()) {
                    Ok(_) => println!("Music opened successfully!"),
                    Err(MusicOpenError::OpenErr) => println!("Music open error!"),
                    Err(MusicOpenError::DecoderErr) => println!("Music decoding error!"),
                    Err(MusicOpenError::SinkErr) => println!("Music sink error!"),
                    _ => {}
                }
            }
        }
    }
}
