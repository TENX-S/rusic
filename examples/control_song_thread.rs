
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::thread::{self, JoinHandle};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Sink, Decoder, OutputStream};

static PAUSED: AtomicBool = AtomicBool::new(true);
static DONE: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
struct Playback {
    inner: Option<JoinHandle<()>>,
}

impl Playback {

    #[inline]
    fn new() -> Self {
        Playback {
            inner: None,
        }
    }

    #[inline]
    fn press(&mut self, song: PathBuf) {
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
                let sound = Decoder::new(BufReader::new(File::open(song).unwrap())).unwrap();
                sink.append(sound);

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

    #[inline]
    fn replace(&mut self, song: PathBuf) {
        DONE.store(true, SeqCst);
        // Blocking for two hundred milliseconds is necessary,
        // because The last thread may be blocking,
        // and we must wait until it sees the DONE
        // to ensure that the thread exit before we can recover the DONE
        thread::sleep(Duration::from_millis(200));
        DONE.store(false, SeqCst);
        PAUSED.store(true, SeqCst);
        self.inner = None;
        self.press(song);
    }

}

fn main() {
    let mut playback = Playback::new();

    println!("Play 3 seconds");
    playback.press(PathBuf::from("/Users/tenx/Music/UltraMusic/Batest.flac")); // play
    thread::sleep(Duration::from_secs(3));

    println!("Pause 2 seconds");
    playback.press(PathBuf::from("/Users/tenx/Music/UltraMusic/Batest.flac")); // pause
    thread::sleep(Duration::from_secs(2));

    println!("Resume 4 seconds");
    playback.press(PathBuf::from("/Users/tenx/Music/UltraMusic/Batest.flac")); // resume
    thread::sleep(Duration::from_secs(4));

    println!("Replace another song");
    playback.replace(PathBuf::from("/Users/tenx/Music/UltraMusic/Beyoncé - If I Were a Boy.mp3")); // switch
    thread::sleep(Duration::from_secs(12));
}
