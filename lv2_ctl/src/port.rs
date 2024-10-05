/// A Port can be:
/// * MIDI in/out.  Unimplemented as yet, here
/// * Audio in/out.
/// * Control
/// * Control can be:
///   * discrete, with a list of values and labels
///   * Continuous.  A continuous port can be:
///     * Integer
///     * Decimal
///     * Float
use crate::mod_host_controller::ScaleDescription;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, PartialOrd, Serialize, Deserialize)]
pub enum ContinuousType {
    Integer,
    Decimal,
    Double,
}

#[derive(Clone, PartialEq, Debug, PartialOrd, Serialize, Deserialize)]
pub struct ContinuousControlPort {
    pub kind: ContinuousType,
    pub max: f64,
    pub min: f64,
    pub default: f64,
    pub logarithmic: bool,
    // When the LV2 is loaded the port values will be loaded.  In
    // useful cases it is an integer or a decimal, but it depends on
    // the type of simulator.  S
    pub value: Option<String>,
}

impl ContinuousControlPort {
    /// ?The 128 values the port can take on
    #[allow(dead_code)]
    pub fn values(&self) -> Vec<String> {
        let range = self.max - self.min;
        let n: usize = 128; // 128 graduations of a MIDI control
        let step = range / n as f64;
        let mut result = vec![];
        for r in 0..n {
            let linear = self.min + r as f64 * step;
            let v = if self.logarithmic {
                linear.exp()
            } else {
                linear
            };
            result.push(format!("{v:0.4}"));
        }
        result
    }
}

#[derive(Clone, PartialEq, Debug, PartialOrd, Serialize, Deserialize)]
pub struct ScaleControlPort {
    /// FIXME! ["Are there default values for these?"];
    pub labels_values: Vec<(String, String)>,
    /// When the LV2 is loaded the port values will be loaded.
    /// Implemented as an index into `labels_values`
    pub value: Option<usize>,
}

impl ScaleControlPort {
    fn _values(&self) -> Vec<&String> {
        self.labels_values
            .iter()
            .map(|(_, a)| a)
            .collect::<Vec<&String>>()
    }

    #[allow(dead_code)]
    fn labels(&self) -> Vec<&String> {
        self.labels_values
            .iter()
            .map(|(a, _a)| a)
            .collect::<Vec<&String>>()
    }
}

#[derive(Clone, PartialEq, Debug, PartialOrd, Serialize, Deserialize)]
pub enum ControlPortProperties {
    Continuous(ContinuousControlPort),
    Scale(ScaleControlPort),
}

impl ControlPortProperties {
    pub fn new(
        min: f64,
        max: f64,
        default: f64,
        logarithmic: bool,
        scale: Option<ScaleDescription>,
        kind: ContinuousType,
    ) -> Self {
        if scale.is_some() {
            let scale = scale.unwrap(); // Safe
            let mut lv: Vec<(String, String)> = vec![];
            for i in 0..scale.labels.len() {
                lv.push((scale.labels[i].clone(), scale.values[i].clone()));
            }
            ControlPortProperties::Scale(ScaleControlPort {
                labels_values: lv,
                value: None,
            })
        } else {
            ControlPortProperties::Continuous(ContinuousControlPort {
                kind,
                min,
                max,
                default,
                logarithmic,
                value: None,
            })
        }
    }
}

//#[derive(, Eq, Hash, Ord,)]
#[derive(Clone, PartialEq, Debug, PartialOrd, Serialize, Deserialize)]
pub enum PortType {
    Input,
    Output,
    Control(ControlPortProperties),
    Audio,
    AtomPort,
    Other(String),
}
impl PortType {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub name: String,   // For display
    pub symbol: String, // For sending to mod-host
    pub types: Vec<PortType>,
    pub index: usize, // index from LV2 description If the simulater is
}

impl Port {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            symbol: "".to_string(),
            types: vec![],
            index: 0,
        }
    }
}
impl Default for Port {
    fn default() -> Self {
        Self::new()
    }
}
