
use std::collections::{HashSet, HashMap};
use super::super::song::Song;

#[derive(Debug)]
pub struct Snapshot {
    cache: HashSet<Song>,
    index: HashMap<u64, Song>,
}
