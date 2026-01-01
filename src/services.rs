use image::{DynamicImage, ImageReader};
use rodio::{OutputStream, Sink, Source};
use std::{
    io::Cursor,
    time::Duration,
};

use crate::models::track::Track;

#[derive(Clone, Copy, Debug)]
pub enum MusicOpenError {
    OpenErr,
    DecoderErr,
    SinkErr,
    // NoTagErr,
    None,
}
pub struct MusicService {
    stream_handle: OutputStream,
    pub music_file: Track,
    sink: Option<Sink>,
    total_duration: Option<Duration>,
}

impl MusicService {
    pub fn new() -> Self {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        Self {
            stream_handle,
            music_file: Track::new("").unwrap(),
            sink: None,
            total_duration: None,
        }
    }

    pub fn open(&mut self, file_path: impl AsRef<std::path::Path>) -> Result<(), MusicOpenError> {
        
        let path = file_path.as_ref();
        self.music_file = Track::new(path).map_err(|_| MusicOpenError::OpenErr)?;
        
        let file = std::fs::File::open(path).map_err(|_| MusicOpenError::OpenErr)?;

        let source = rodio::Decoder::try_from(file).map_err(|_| MusicOpenError::DecoderErr)?;

        self.total_duration = source.total_duration();

        let sink = rodio::Sink::connect_new(self.stream_handle.mixer());

        sink.append(source);
        self.sink = Some(sink);

        Ok(())
    }
    pub fn resume(&self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }
    pub fn stop(&self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }
    pub fn pause(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    pub fn decode_image(&self) -> image::ImageResult<DynamicImage> {
        let bytes = self.music_file.extract_img_bytes()?;
        let bytes: &[u8] = &bytes;
        ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()
    }

    pub fn is_music_loaded(&self) -> bool {
        self.sink.is_some()
    }

    pub fn set_pos(&self, pos: Duration) {
        if let Some(sink) = self.sink.as_ref() {
            if let Err(e) = sink.try_seek(pos) {
                eprintln!("Error seeking to {:?}: {:?}", pos, e);
            }
        }
    }
    pub fn get_pos(&self) -> Option<Duration> {
        let pos = self.sink.as_ref()?.get_pos();
        Some(pos)
    }

    pub fn get_total_duration(&self) -> Option<Duration> {
        self.total_duration
    }
}
