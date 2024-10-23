//!  Handle the MIDI connections
use std::error;
// use std::fmt;
use std::thread;
use std::time::Duration;

#[derive(Default)]
pub struct MidiData {
    pub connection_cache: Vec<(String, String)>,
    pub last: u8,
}
// #[derive(Debug, Clone)]
// struct MidiError {
//     what: String,
// }

// impl fmt::Display for MidiError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "120Proof MIDI Error: {}", self.what)
//     }
// }
// impl error::Error for MidiError {}
pub struct Midi {
    pub name: String,
}

impl Midi {
    pub fn new(name: String) -> Result<Self, Box<dyn error::Error>> {
        Ok(Midi { name })
    }

    pub fn run(
        &self,
        mut f: impl FnMut(&[u8], &mut MidiData) + Send + 'static,
    ) -> Result<(), Box<dyn error::Error>> {
        // TODO: Should allow name to be controlled fom command line.
        // May be more than one pedal in use.
        let this_name = "120Proof_pedal".to_string();
        let midi_in = midir::MidiInput::new(this_name.as_str())?;
        for (index, port) in midi_in.ports().iter().enumerate() {
            // Each available input port.
            match midi_in.port_name(port) {
                Err(_) => continue,
                Ok(port_name) => {
                    eprintln!("DEBUGGING: port_name: {port_name}");
                    if port_name.as_str().contains(self.name.as_str()) {
                        // Found the port (first port that `card_name`
                        // is a subset of)

                        let this_port = midi_in
                            .ports()
                            .get(index)
                            .ok_or("Invalid port number")
                            .unwrap()
                            .clone();

                        let connect = midi_in.connect(
                            &this_port,
                            format!("{}-in", this_name).as_str(),
                            move |_a, b, connection_cache| {
                                let c = if b[1] > 3 && b[1] <= 7 {
                                    b[1] - 4
                                } else if b[1] > 7 && b[1] <= 11 {
                                    b[1] - 8
                                } else if b[1] > 11 && b[1] <= 15 {
                                    b[1] - 12
                                } else if b[1] > 15 && b[1] <= 19 {
                                    b[1] - 16
                                } else if b[1] > 19 && b[1] <= 23 {
                                    b[1] - 20
                                } else if b[1] > 23 && b[1] <= 27 {
                                    b[1] - 24
                                } else if b[1] > 27 && b[1] <= 31 {
                                    b[1] - 28
                                } else {
                                    b[1]
                                };
                                println!("MIDI in {:?}/{c}", &b);

                                f(&[192, c], connection_cache);
                            },
                            MidiData::default(),
                        );
                        match connect {
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
