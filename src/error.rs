pub use anyhow::{anyhow, Result};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Unknown")]
pub struct Unknown;
