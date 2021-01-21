use super::format::Format::{self, *};
use crate::error::Result;
use std::path::Path;
use id3::Tag as MP3Tag;
use metaflac::Tag as FLACTag;
use crate::utils::{get_duration, display_duration};

#[derive(Debug, Clone)]
pub struct Metadata<T=String> {
    pub title: Option<T>,
    pub artist: Option<T>,
    pub album: Option<T>,
    pub format: Format,
    pub duration: Option<f64>,
}

impl Metadata {

    #[inline]
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {

        let mut title = None;
        let mut artist = None;
        let mut album= None;
        let format = Format::new(&path)?;
        let mut duration = None;

        match format {
            FLAC => {
                let tag = FLACTag::read_from_path(&path)?;
                if let Some(vbscmt) = tag.vorbis_comments() {
                    title = vbscmt.title().map(|v| v.join(" "));
                    artist = vbscmt.artist().map(|v| v.join(" "));
                    album = vbscmt.album().map(|v| v.join(" "));
                    duration = get_duration(&path).ok();
                }
            }
            MP3 => {
                let tag = MP3Tag::read_from_path(&path)?;
                title = tag.title().map(str::to_string);
                artist = tag.artist().map(str::to_string);
                album = tag.album().map(str::to_string);
                duration = tag.duration().map(|t| t as f64);
                if duration.is_none() {
                    duration = get_duration(&path).ok();
                }
            }
            WAV => {
                todo!()
            }
            OGG => {
                todo!()
            }
            Unsupported => {

            }
        }

        Ok(Metadata {
            title,
            artist,
            album,
            format,
            duration,
        })

    }

    #[inline]
    pub fn header(&self) -> Vec<String> {
        let unknown = || "Unknown".to_owned();

        vec![
            self.title.clone().unwrap_or_else(unknown),
            self.artist.clone().unwrap_or_else(unknown),
            self.album.clone().unwrap_or_else(unknown),
            display_duration(self.duration),
        ]

    }

}
