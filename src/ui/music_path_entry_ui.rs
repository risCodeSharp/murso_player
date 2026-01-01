use std::{fs, path::PathBuf};

use crate::{
    services::{MusicOpenError, MusicService},
    ui::music_list,
};

use eframe::egui::{self, ComboBox, Response, TextureHandle, Ui};

pub struct MusicPathEntryUI {
    path: String,
    directory_search_reponse: Option<Response>,
    pub request_load_music: bool,
    pub music_list: Vec<String>,
    pub selected_music: Option<String>,
}

impl MusicPathEntryUI {
    pub fn new() -> Self {
        Self {
            path: String::from("music/audio.mp3"),
            directory_search_reponse: None,
            request_load_music: false,
            music_list: Vec::new(),
            selected_music: None,
        }
    }
pub fn show(&mut self, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        // --- Search Bar ---
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                self.directory_search_reponse = Some(ui.button("🔍 Find"));
                ui.add(
                    egui::TextEdit::singleline(&mut self.path)
                        .hint_text("Enter music directory...")
                        .desired_width(ui.available_width()),
                );
            });
        });

        ui.add_space(5.0);

        // --- Music List ---
        if !self.music_list.is_empty() {
            ui.label(egui::RichText::new("Library").strong());
            
            egui::ScrollArea::vertical()
                .max_height(250.0) // Limits height so player is visible
                .show(ui, |ui| {
                    for (index, music) in self.music_list.iter().enumerate() {
                        let is_selected = self.selected_music.as_ref() == Some(music);
                        
                        // FIX: push_id prevents the "ID" render error
                        ui.push_id(index, |ui| {
                            if ui.selectable_label(is_selected, format!("🎵 {}", music)).clicked() {
                                self.selected_music = Some(music.clone());
                                self.request_load_music = true;
                            }
                        });
                    }
                });
        }
    });
}
    fn music_files_from_dir(&self) -> Option<Vec<String>> {
        let path = std::path::Path::new(&self.path);
        if !path.exists() {
            println!("Directory doesn't exists");
            return None;
        }

        if !path.is_dir() {
            println!("Path is not directory");
            return None;
        }
        println!("Hello");
        let list: Vec<String> = fs::read_dir(path)
            .unwrap()
            .into_iter()
            .map(|entry| match entry {
                Ok(entry) => {
                    if let Some(name) = entry.file_name().to_str() {
                        name.to_string()
                    } else {
                        eprintln!("Invalid UTF-8 in file name");
                        String::new()
                    }
                }
                Err(e) => {
                    eprintln!("Directory entry read error: {e}");
                    String::new()
                }
            })
            .collect();

        Some(list)
    }

pub fn on_submit(
    &mut self,
    _ctx: &egui::Context,
    service: &mut MusicService,
    _texture_handle: &mut Option<TextureHandle>,
) {
    // 1. Handle Directory Searching
    if let Some(response) = &self.directory_search_reponse {
        if response.clicked() {
            println!("Hello");
            if let Some(music_files) = self.music_files_from_dir() {
                self.music_list = music_files;
            }
        }
    }

    // 2. Handle File Loading
    // This must check the flag set by the 'show' method's click event
    if self.request_load_music {
        if let Some(music_file) = &self.selected_music {
            let music_path = PathBuf::from(&self.path).join(music_file);
            
            match service.open(&music_path) {
                Ok(_) => {
                    println!("Successfully loaded: {:?}", music_path);
                    self.request_load_music = false; // Reset ONLY after successful load
                }
                Err(e) => {
                    eprintln!("Failed to load music: {:?}", e);
                    self.request_load_music = false; // Reset even on error to stop loop
                }
            }
        }
    }
}
}
