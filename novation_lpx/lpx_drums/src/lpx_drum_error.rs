use crate::section::Section;
use std::error::Error;
use std::fmt;
/// The errors that can be generated in LpxDrums

#[derive(Debug)]
pub enum LpxDrumError {
    InvalidSection,
    IntersectingSections(Section, Section),
    DuplicateMainColour(Section, Section),
    DuplicateMIDI(Section, Section),
}

impl fmt::Display for LpxDrumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LpxDrumError::InvalidSection => write!(f, "invalid section"),
            LpxDrumError::IntersectingSections(s1, s2) => {
                write!(f, "intersecting sections: {} {}", s1, s2)
            }
            LpxDrumError::DuplicateMainColour(s1, s2) => {
                write!(f, "duplicate main colour: {} {}", s1, s2)
            }
            LpxDrumError::DuplicateMIDI(s1, s2) => {
                write!(f, "duplicate MIDI: {} {}", s1, s2)
            }
        }
    }
}

impl Error for LpxDrumError {}
