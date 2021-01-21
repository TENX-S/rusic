use std::fs::File;
use std::sync::Arc;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;
use std::thread::{self, JoinHandle};
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use super::super::song::Song;
use super::super::player::CURRENT;
use rodio::{Sink, Decoder, OutputStream};

/// A flag to control or display the pause status of the current song
static PAUSED: AtomicBool = AtomicBool::new(true);

/// A flag to control or display the ending status of the current song
static DONE: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub struct Playback {
    last: Option<u64>,
    inner: Option<JoinHandle<()>>,
}

impl Default for Playback {
    #[inline]
    fn default() -> Self {
        Playback {
            last: None,
            inner: None,
        }
    }
}

impl Playback {

    /// Play the song in the current player,
    /// pause/continue if it is the same as the last one,
    /// otherwise switch.
    #[inline]
    pub fn handle(&mut self, current: &Song) {
        if let Some(last) = self.last {
            if current.id == last {
                self.press(current);
            } else {
                self.last = Some(current.id);
                self.switch(current);
            }
        } else {
            self.last = Some(current.id);
            self.press(current);
        }
    }

    /// "Press the play button".
    /// Play or pause a song
    #[inline]
    pub fn press(&mut self, song: &Song) {
        let song = song.clone();
        if self.inner.is_some() {
            if PAUSED.load(SeqCst) {
                PAUSED.store(false, SeqCst);
            } else {
                PAUSED.store(true, SeqCst);
            }
        } else {
            PAUSED.store(false, SeqCst);
            self.inner = Some(thread::spawn(move || {
                let (_stream, handle) = OutputStream::try_default().unwrap();
                let sink = Arc::new(Sink::try_new(&handle).unwrap());

                // TODO: Error handling
                // Maybe to pop up a prompt window?
                // Or to make sure the path here is always valid
                if let Ok(file) = File::open(&song.pos) {
                    if let Ok(sound) = Decoder::new(BufReader::new(file)) {
                        sink.append(sound);
                    }
                }

                let notify_done = Arc::clone(&sink);
                let _natural_end = thread::spawn(move || {
                    notify_done.sleep_until_end();
                    DONE.store(true, SeqCst);
                });

                loop {
                    // Don't keep the CPU too busy
                    thread::sleep(Duration::from_millis(100));
                    if PAUSED.load(SeqCst) {
                        sink.pause();
                    } else {
                        sink.play();
                    }
                    if DONE.load(SeqCst) {
                        break;
                    }
                }
            }));

        }
    }

    /// End the last song,
    /// switch to the specified song
    #[inline]
    pub fn switch(&mut self, song: &Song) {
        DONE.store(true, SeqCst);
        // Blocking for two hundred milliseconds is necessary,
        // because the last thread may be blocked,
        // we must wait until it sees the `DONE`
        // to ensure that the thread exit before we can recover the `DONE`
        thread::sleep(Duration::from_millis(200));
        DONE.store(false, SeqCst);
        PAUSED.store(true, SeqCst);
        self.inner = None;
        self.press(song);
    }

}
