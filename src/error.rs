use console::style;
use std::fmt;

use crate::constants::*;

#[derive(Debug)]
pub enum ArtGenError {
    MissingDirectory(String),
    InvalidCollectionSize,
    InsufficientLayers,
    IncorrectDirectoryConvention(String),
    IncorrectFileConvention(String),
}

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
            Self::InsufficientLayers => {
                write!(
                    f,
                    "\n\n{}{} ",
                    ERROR_EMOJI,
                    style("Not enough layers for requested collection size")
                        .red()
                        .bold(),
                )
            }
            Self::IncorrectDirectoryConvention(directory) => {
                write!(
                    f,
                    "\n\n{}{} `{}` {}",
                    ERROR_EMOJI,
                    style("Incorrect convention for directory:").red().bold(),
                    directory,
                    style("Make sure the directory is of form <weight><directory_name>").red(),
                )
            }
            Self::IncorrectFileConvention(directory) => {
                write!(
                    f,
                    "\n\n{}{} `{}` {}",
                    ERROR_EMOJI,
                    style("Incorrect convention for file:").red().bold(),
                    directory,
                    style("Make sure the file is of form <weight><file_name>").red(),
                )
            }
        }
    }
}

// pub type Result<T> = std::result::Result<T, ArtGenError>;
