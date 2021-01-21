use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use rodio::{Decoder, Source};
use crate::error::{Result, Unknown, anyhow};

#[inline]
pub fn get_duration(path: impl AsRef<Path>) -> Result<f64> {
    Decoder::new(
        BufReader::new(
            File::open(&path)?
        )
    )?
        .total_duration()
        .map(|t| t.as_secs() as f64)
        .ok_or(anyhow!(Unknown))
}

#[inline]
pub fn display_duration(duration: Option<f64>) -> String {

    let mut result = "Unknown".to_owned();
    if let Some(duration) = duration {

        let mut hour = 0.0;
        let mut minutes = 0.0;
        let seconds;
        if duration > 3600.0 {
            hour = duration / 3600.0;
            minutes = (duration - 3600.0 * hour) / 60.0;
            seconds = (duration - 3600.0 * hour) % 60.0;
        } else if duration > 60.0 {
            minutes = duration / 60.0;
            seconds = duration % 60.0;
        } else {
            seconds = duration;
        }

        if hour != 0.0 {
            result = format!("{:02}:{:02}:{:02}", hour as u64, minutes as u64, seconds as u64);
            return result;
        }
        if minutes != 0.0 {
            result = format!("{:02}:{:02}", minutes as u64, seconds as u64);
            return result;
        }
        if seconds != 0.0 {
            result = format!("00:{:02}", seconds as u64);
            return result;
        }
    }

    result

}
