use std::error::Error;
use std::fmt;
/// The errors that can be generated in LpxDrums

#[derive(Debug)]
pub enum LpxDrumError {
    InvalidSection,
}

impl fmt::Display for LpxDrumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LpxDrumError::InvalidSection => write!(f, "invalid section"),
        }
    }
}

impl Error for LpxDrumError {
    fn description(&self) -> &str {
        match *self {
            LpxDrumError::InvalidSection => "Invalid section",
        }
    }

    fn cause(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            LpxDrumError::InvalidSection => None,
        }
    }
}
