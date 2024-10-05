//! Definition of an LV2 simulator as defined in the Turtle files
use crate::port::ContinuousType;
use crate::port::ControlPortProperties;
use crate::port::Port;
use crate::port::PortType;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The assembled simulator with all the data necessary to load it
/// into a host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lv2 {
    pub types: HashSet<Lv2Type>,
    pub ports: Vec<Port>,
    pub name: String,
    pub url: String,
}

#[derive(
    PartialEq, Eq, Hash, Debug, Ord, PartialOrd, Clone, Serialize, Deserialize,
)]
pub enum Lv2Type {
    Plugin,
    ReverbPlugin,
    ChorusPlugin,
    FlangerPlugin,
    PhaserPlugin,
    WaveshaperPlugin,
    FunctionalProperty,
    SpectralPlugin,
    LimiterPlugin,
    AnalyserPlugnin,
    PitchPlugin,
    ObjectProperty,
    Property,
    SpatialPlugin,
    UtilityPlugin,
    AmplifierPlugin,
    ExpanderPlugin,
    CompressorPlugin,
    EQPlugin,
    ModulatorPlugin,
    InstrumentPlugin,
    SimulatorPlugin,
    FilterPlugin,
    DelayPlugin,
    DistortionPlugin,
    Class,
    Project,
    EnvelopePlugin,
    Other(String),
}

/// Stores all the data required to run LV2 simulators
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Lv2Datum {
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

/// Unicode constants for display
const LESSEQ: &str = "\u{2a7d}"; // <=
const LOG: &str = "\u{33d2}"; // log

impl fmt::Display for ControlPortProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ControlPortProperties::Continuous(c) => {
                    format!(
                        "{} {} {LESSEQ} {}  {LESSEQ} {} {}",
                        match c.kind {
                            ContinuousType::Integer => "Int",
                            ContinuousType::Decimal => "Dec",
                            ContinuousType::Double => "F  ",
                        },
                        c.min,
                        c.default,
                        c.max,
                        if c.logarithmic { LOG } else { "" }
                    )
                }
                ControlPortProperties::Scale(s) => format!(
                    "Scale: {}",
                    s.labels_values.iter().fold(String::new(), |a, b| format!(
                        "{a} {}/{}",
                        b.0, b.1
                    ))
                ),
            }
        )
    }
}
impl fmt::Display for PortType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortType::Input => write!(f, "Input"),
            PortType::Output => write!(f, "Output"),
            PortType::Control(properties) => {
                write!(f, "Control({})", properties)
            }
            PortType::Audio => write!(f, "Audio"),
            PortType::AtomPort => write!(f, "AtomPort"),
            PortType::Other(s) => write!(f, "Other({})", s),
        }
    }
}
impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let port_types: Vec<String> =
            self.types.iter().map(|t| format!("{}", t)).collect();
        write!(
            f,
            "Port {}: {} [{}]",
            self.index,
            self.name,
            port_types.join(", "),
        )
    }
}
impl fmt::Display for Lv2Datum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.subject, self.predicate, self.object)
    }
}
impl fmt::Display for Lv2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}{}{}",
            self.name,
            self.url,
            self.ports
                .iter()
                .fold("".to_string(), |a, b| format!("{}\n\t{}", a, b)),
            self.types
                .iter()
                .fold("".to_string(), |a, b| format!("{}\n\t{:?}", a, b))
        )
    }
}
