
# 🎵 Sumo Music Player

A lightweight, responsive music player built with **Rust** and the **egui/eframe** framework. This player features a directory-based music library, real-time playback synchronization, and a clean, centered UI that adapts to window resizing.

## ✨ Features

* **Directory-Based Library**: Easily load music files by searching specific directories on your system.
* **Persistent Controls**: Play, Pause, and Stop buttons remain accessible in a dedicated bottom panel.
* **Real-Time Seeking**: A smooth timeline slider that updates as the song plays and allows for manual seeking (drag/click).
* **Responsive Layout**: Perfectly centered playback controls that remain in the middle of the window even when resized.
* **Robust ID Handling**: Implements scoped IDs (`push_id`) to prevent UI rendering collisions in lists.

---

## 🛠️ Technical Stack

* **Language**: [Rust](https://www.rust-lang.org/)
* **UI Framework**: [egui](https://github.com/emilk/egui) & [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
* **Audio Backend**: Custom `MusicService` implementation.
* **File Handling**: `std::fs` and `std::path`.

---

## 🚀 Getting Started

### Prerequisites

Ensure you have the Rust toolchain installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

### Running the Application

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-egui-music-player.git
cd rust-egui-music-player

```


2. Build and run:
```bash
cargo run --release

```



---

## 🏗️ Architecture

The application is structured into modular UI components to keep the main loop clean:

| Component | Responsibility |
| --- | --- |
| `MusicPlayer` | Main `eframe::App` loop and panel layout management. |
| `MusicPathEntryUI` | Handles directory searching and the scrollable library list. |
| `MusicButtonUI` | Renders styled buttons and the playback timeline slider. |
| `MusicService` | The backend logic for audio decoding and playback state. |

---

## 💡 Implementation Highlights

### Resizable Centering Logic

To achieve perfect centering during window resizes, the player utilizes `allocate_ui_with_layout` with a centered main alignment, avoiding fragile manual coordinate calculations.

```rust
ui.allocate_ui_with_layout(
    egui::vec2(ui.available_width(), 30.0),
    egui::Layout::left_to_right(egui::Align::Center).with_main_align(egui::Align::Center),
    |ui| {
        // Buttons stay centered here...
    }
);

```

### Preventing ID Collisions

When rendering the music list, we use `enumerate()` and `push_id` to ensure `egui` uniquely identifies every track, even if filenames are identical.

> **Note**: This prevents the "ID" text overlay often seen in debug renders of egui applications.

---

## 📝 License

Distributed under the MIT License. See `LICENSE` for more information.

---

**Would you like me to add a "Roadmap" section to this README with ideas like "Playlist Support" or "Volume Normalization"?**
