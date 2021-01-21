mod snapshot;

use std::path::{PathBuf};
use snapshot::Snapshot;
use super::song::Song;


#[derive(Debug)]
pub struct Library {
    pub pos: PathBuf,
    pub snapshot: Snapshot,
}

const SUPPORTED_FORMAT: &[&str] = &["[!.]*.flac", "[!.]*.mp3", "[!.]*.wav", "[!.]*.ogg"];
