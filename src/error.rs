use console::style;
use std::fmt;

use crate::constants::*;

#[derive(Debug)]
pub enum ArtGenError {
    MissingDirectory(String),
    InvalidCollectionSize,
}
/*
impl ArtGenError {
    pub fn output(&self) -> String {
        match self {
            ArtGenError::MissingDirectory(directory) => {
                format!("Could not locate directory `{:?}`", directory)
            }
            ArtGenError::NonNegativeNumberRequired => {
                "Non-negative number required for collection size".into()
            }
        }
    }
}
*/
impl fmt::Display for ArtGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDirectory(directory) => {
                write!(
                    f,
                    "\n\n{}{} `{}`",
                    ERROR_EMOJI,
                    style("Could not locate directory:").red().bold(),
                    directory
                )
            }
            Self::InvalidCollectionSize => {
                write!(
                    f,
                    "\n\n{}{} ",
                    ERROR_EMOJI,
                    style("Collection size `n` must be a positive integer value")
                        .red()
                        .bold(),
                )
            }
        }
    }
}

// pub type Result<T> = std::result::Result<T, ArtGenError>;
