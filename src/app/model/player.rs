mod mode;
mod playback;

use super::song::Song;
use mode::Mode;
use playback::Playback;
use parking_lot::Mutex;
use once_cell::sync::Lazy;

/// The currently selected song
pub static CURRENT: Lazy<Mutex<Option<Song>>> = Lazy::new(|| {
    Mutex::new(None)
});

#[derive(Debug)]
pub struct Player {
    pub mode: Mode,
    pub playback: Playback,
    pub progress: Option<f64>,
}

impl Default for Player {
    #[inline]
    fn default() -> Self {
        Player {
            mode: Default::default(),
            playback: Default::default(),
            progress: None,
        }
    }
}

impl Player {

    #[inline]
    fn play(&mut self) {
        if let Some(song) = CURRENT.lock().as_ref() {
            self.playback.handle(song);
        }
    }

    #[inline]
    fn ratio(&self) -> f64 {
        0.0
    }

}

#[inline]
pub fn pick() {

}
