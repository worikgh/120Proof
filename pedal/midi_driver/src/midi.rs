//!  Handle the MIDI connections
use std::error;
use std::fmt;
use std::thread;
use std::time::Duration;

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
    pub name: String,
    // Cache connections that are made so they can be deleted after
    // other connections made
    pub connection_cache: Option<Vec<(String, String)>>,
}

impl Midi {
    pub fn new(name: String) -> Result<Self, Box<dyn error::Error>> {
        Ok(Midi {
            name,
            connection_cache: None,
        })
    }

    pub fn run(
        &self,
        mut f: impl FnMut(&[u8], &mut Vec<(String, String)>) + Send + 'static,
    ) -> Result<(), Box<dyn error::Error>> {
        let this_name = "120Proof_pedal".to_string();
        let midi_in = midir::MidiInput::new(this_name.as_str())?;
        let connection_cache: Vec<(String, String)> = vec![];
        for (index, port) in midi_in.ports().iter().enumerate() {
            // Each available input port.
            match midi_in.port_name(port) {
                Err(_) => continue,
                Ok(port_name) => {
                    println!("DEBUGGING: port_name: {port_name}");
                    if port_name.as_str().contains(self.name.as_str()) {
                        // Found the port (first port that `card_name`
                        // is a subset of)

                        let this_port = midi_in
                            .ports()
                            .get(index)
                            .ok_or("Invalid port number")
                            .unwrap()
                            .clone();

                        _ = match midi_in.connect(
                            &this_port,
                            format!("{}-in", this_name).as_str(),
                            move |_a, b, connection_cache| {
                                println!("MIDI in {:?}", &b);
                                f(b, connection_cache);
                            },
                            connection_cache,
                        ) {
                            Ok(_) => {
                                println!("Created MIDI in");
                                loop {
                                    thread::sleep(Duration::from_secs(1));
                                }
                            }
                            Err(err) => {
                                println!("Could not connect {:?}", err);
                            }
                        };
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
