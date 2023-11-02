extern crate midir;

use crate::midir::os::unix::VirtualOutput;
use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::error::Error;

use crate::section::Section;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::sync::mpsc::{self, Receiver, Sender};
mod lpx_drum_error;
mod section;
fn main() -> Result<(), Box<dyn Error>> {
    // The only argument is a configuration file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Pass name of drum definition, JSON formatted, file as sole argument");
    }
    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Creatre the sections from the file
    let result: Vec<Section> = Section::parse_json(&content)?;
    if let Err(err) = Section::check_sections(&result) {
        panic!("{err}");
    }

    // Port to read MIDI from LPX
    let lpx_midi = MidiInput::new("LpxDrumMidiIn")?;
    let in_ports = lpx_midi.ports();
    let in_port = in_ports.get(0).ok_or("no input port available")?;

    // The channel to send MIDI messages from the MidiInputConnection here
    let (tx, rx): (Sender<[u8; 3]>, Receiver<[u8; 3]>) = mpsc::channel::<[u8; 3]>();

    let _conn_in: MidiInputConnection<Sender<[u8; 3]>> = lpx_midi.connect(
        in_port,
        "read_input",
        move |_stamp, message, tx| {
            // let message = MidiMessage::from_bytes(message.to_vec());
            if message.len() == 3 {
                let message: [u8; 3] = message.try_into().unwrap();
                tx.send(message).unwrap();
            }
        },
        tx.clone(),
    )?;

    let midi_out: MidiOutput = MidiOutput::new("LpxDrum")?;
    let port_name = "LpxDrumMidiOut";
    let mut out_port: MidiOutputConnection = midi_out.create_virtual(port_name)?;
    eprintln!("Virtual MIDI Output port '{port_name}' is open");

    // // Initialise the pad clours

    loop {
        let message: [u8; 3] = match rx.recv() {
            Ok(m) => m,
            Err(err) => panic!("{}", err),
        };

        if message[0] == 144 {
            // All MIDI notes from LPX start with 144
            let velocity = message[2];
            if velocity > 0 {
                // Note on.
                eprintln!("In main loop Send note: {message:?}");
                out_port.send(&message)?;
            }
        }
    }
    // Ok(())
}
