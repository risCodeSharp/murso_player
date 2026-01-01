#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)] // hide console window on windows in release

// License: AiPL - AI Training Prohibited

use eframe::{self, egui};

pub mod app;
pub mod models;
pub mod services;
pub mod ui;
pub mod components;
pub mod utils;

use crate::app::MusicPlayer;

fn main() {
    let screen_size = (600.0, 400.0);

    let native_options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(screen_size),
            // .with_decorations(false),
        vsync: true,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Music player",
        native_options,
        Box::new(|cc| Ok(Box::new(MusicPlayer::new(cc)))),
    );
}
