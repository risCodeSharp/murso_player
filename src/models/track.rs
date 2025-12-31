use id3::frame::PictureType;
use id3::{Tag, TagLike};
use image::ImageError;
use std::{
    fs::File,
    io::{self, BufReader, Result},
    path::{Path, PathBuf},
};

#[derive(Copy, Clone)]
pub enum MusicState {
    NotStarted,
    Playing,
    Stopped,
    Completed,
}
#[derive(Clone)]
pub struct Track {
    path: PathBuf,
    state: MusicState,
    tag: Option<Tag>,
}

impl Track {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if !std::fs::exists(&path)? {
            return Ok(Self {
                path,
                tag: None,
                state: MusicState::NotStarted,
            });
        }

        let tag = match Tag::read_from_path(&path) {
            Ok(v) => {
                println!("Tag is created");
                Some(v)
            }
            Err(e) => {
                println!("Failed to create tag! {e}");
                None
            }
        };

        Ok(Self {
            path,
            tag,
            state: MusicState::NotStarted,
        })
    }

    pub fn set_state(&mut self, state: MusicState) {
        self.state = state
    }

    pub fn state(&self) -> MusicState {
        self.state
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn to_reader(&self) -> io::Result<BufReader<File>> {
        Ok(BufReader::new(File::open(&self.path)?))
    }
    pub fn name(&self) -> String {
        if let Some(tag) = &self.tag {
            if let Some(name) = tag.title() {
                name.to_string();
            }
        }
        Path::new(&self.path)
            .file_stem()
            .expect("Invalid Path")
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or(String::from("Not named song"))
    }

    pub fn artist(&self) -> String {
        if let Some(tag) = &self.tag {
            if let Some(artist) = tag.artist() {
                artist.to_string();
            }
        }
        String::from("No named artist")
    }

    pub fn extract_img_bytes(&self) -> image::ImageResult<Vec<u8>> {
        let tag = self.tag.as_ref().ok_or_else(|| {
            ImageError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Tag not found",
            ))
        })?;
        let picture = tag
            .pictures()
            .find(|p| p.picture_type == PictureType::CoverFront)
            .ok_or_else(|| {
                ImageError::IoError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No front cover picture",
                ))
            })?;
        Ok(picture.data.clone())
    }
}
