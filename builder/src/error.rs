//! A tiny error type so the crate stays dependency-free.

use std::fmt;

pub type Result<T> = std::result::Result<T, BuildError>;

#[derive(Debug)]
pub struct BuildError {
    message: String,
}

impl BuildError {
    pub fn new(message: impl Into<String>) -> BuildError {
        BuildError {
            message: message.into(),
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for BuildError {}

impl From<std::io::Error> for BuildError {
    fn from(e: std::io::Error) -> Self {
        BuildError::new(e.to_string())
    }
}
