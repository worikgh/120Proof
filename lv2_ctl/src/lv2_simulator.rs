//! The information needed to display a loaded LV2 simulator in a list.
use serde::{Deserialize, Serialize};

/// Whether the simuator is loaded into mod-host
#[derive(Copy, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Status {
    Loaded,
    Pending,
    Unloaded,
}

/// The control ports need to be displayed.  They need a value a value
/// to display and adjust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ControlPort {
    /// The name that describes this port to mod-host
    pub param_symbol: String,

    // The values that can be sent to the backend.  Port values can be
    // symbolic, integer, or decimal, but they are allways sent as
    // strings.
    pub value: Option<String>,

    /// If it is a ScalePort it has a label as well as a value
    pub label: Option<Vec<String>>,
}

/// All the information the front end needs to control a LV2
/// simulator.  Most of the data is stored as String, that is how it
/// is sent to and received from mod-host
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lv2Simulator {
    /// Display name
    pub name: String,

    /// Unique identifier.  There can be more than one Lv2Simulator in
    /// a list, each with a different `mh_id`.
    pub url: String,

    /// Status loaded, unloaded, or pending
    pub status: Status,

    /// The number assigned to this simulator for mod-host
    pub mh_id: usize,
}

impl Lv2Simulator {}

/// The values the port can take.
/// The maximum is 128 discreet values for MIDI control

/// Create a `Lv2Simulator` two Strings: name, url, and a `Status`
impl From<&(String, String, Status)> for Lv2Simulator {
    fn from((name, url, status): &(String, String, Status)) -> Self {
        Self {
            name: name.clone(),
            status: *status,
            url: url.clone(),
            mh_id: 0,
        }
    }
}
