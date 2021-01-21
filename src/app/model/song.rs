mod format;
mod metadata;

use metadata::Metadata;
use std::path::PathBuf;

type ID = u64;

#[derive(Debug, Clone)]
pub struct Song {
    pub id: ID,
    pub pos: PathBuf,
    pub metadata: Metadata,
}
