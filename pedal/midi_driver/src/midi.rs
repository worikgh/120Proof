//!  Handle the MIDI connections
use std::error;

use std::fmt;

// type Result<T> = std::result::Result<T, MidiError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct MidiError {
    what: String,
}

// Generation of an error is completely separate from how it is
// displayed.  There's no need to be concerned about cluttering
// complex logic with the display style.
//
// Note that we don't store any extra info about the errors.
impl fmt::Display for MidiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "120Proof MIDI Error: {}", self.what)
    }
}
impl error::Error for MidiError {}
pub struct Midi {
    pedal_port: Option<midir::MidiInputConnection<()>>,
    pub name: String,
}

impl Midi {
    pub fn new(name: String) -> Result<Self, Box<dyn error::Error>> {
        let mut pedal_port: Option<midir::MidiInputConnection<()>> = None;
        let this_name = "120Proof_pedal".to_string();
        let midi_in = midir::MidiInput::new(this_name.as_str())?;
        for (index, port) in midi_in.ports().iter().enumerate() {
            // Each available input port.
            match midi_in.port_name(port) {
                Err(_) => continue,
                Ok(port_name) => {
                    println!("DEBUGGING: port_name: {port_name}");
                    if port_name.as_str().contains(name.as_str()) {
                        // Found the port (first port that `card_name`
                        // is a subset of)
                        let this_port = midi_in
                            .ports()
                            .get(index)
                            .ok_or("Invalid port number")
                            .unwrap()
                            .clone();
                        pedal_port = match midi_in.connect(
                            &this_port,
                            format!("{}-in", this_name).as_str(),
                            |a, b, _| {
                                Self::handle_midi(a, b);
                            },
                            (),
                        ) {
                            Ok(s) => Some(s),
                            Err(err) => {
                                eprintln!("Could not connect {:?}", err);
                                None
                            }
                        };
                        break;
                    }
                }
            }
        }
        Ok(Midi { pedal_port, name })
    }
    fn handle_midi(a: u64, b: &[u8]) {
        println!("{a} {:?}", b);
    }
}

/// Return a vector of device identifiers
pub fn list_device_names() -> Vec<String> {
    vec![]
}
