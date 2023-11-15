extern crate midir;
mod lpx_drum_error;
mod section;

use crate::midir::os::unix::VirtualOutput;
use crate::section::Section;
use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::sync::mpsc::{self, Receiver, Sender};

fn load_sections(filename: &str) -> Result<Vec<Section>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Creatre the sections from the file
    let result: Vec<Section> = Section::parse_json(&content)?;
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    // The only argument is a configuration file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Pass name of drum definition, JSON formatted, file as sole argument");
    }
    let filename = &args[1];
    let sections: Vec<Section> = load_sections(filename)?;

    // Port to read MIDI from LPX
    let lpx_midi = MidiInput::new("LpxDrumMidiIn")?;
    let in_ports = lpx_midi.ports();
    let in_port = in_ports.get(0).ok_or("no input port available")?;

    // The channel to send MIDI messages, received from the LPX in the
    // MidiInputConnection, here to the main thread
    let (tx, rx): (Sender<[u8; 3]>, Receiver<[u8; 3]>) = mpsc::channel::<[u8; 3]>();

    // `_conn_in` holds the port the LPX will connect to
    let _conn_in: MidiInputConnection<Sender<[u8; 3]>> = lpx_midi.connect(
        in_port,
        "read_input",
        move |_stamp, message, tx| {
            // let message = MidiMessage::from_bytes(message.to_vec());
            // eprintln!("MIDI From LPX: {message:?}");
            if message.len() == 3 {
                let message: [u8; 3] = message.try_into().unwrap();
                tx.send(message).unwrap();
            }
        },
        tx.clone(),
    )?;

    // Establish the output to the drum sythesiser that will make the
    // drum sounds
    let midi_out: MidiOutput = MidiOutput::new("LpxDrumNote")?;
    let port_name = "port";
    let mut midi_note_out_port: MidiOutputConnection = midi_out.create_virtual(port_name)?;
    // eprintln!("Virtual MIDI Output port '{port_name}' is open");

    // Establish the output for control signals
    let midi_out: MidiOutput = MidiOutput::new("LpxDrumCtl")?;
    let port_name = "port";
    let mut midi_ctl_out_port: MidiOutputConnection = midi_out.create_virtual(port_name)?;
    // eprintln!("Virtual MIDI Output port '{port_name}' is open");

    // Create an output port to the LPX for sending it colour.
    let colour_out: MidiOutput = MidiOutput::new("LpxDrum")?;
    let port_name = "LpxDrumColourOut";
    let mut colour_port: MidiOutputConnection = colour_out.create_virtual(port_name)?;
    // eprintln!("Virtual MIDI Output port '{port_name}' is open");

    // Initialise the LPX
    let msg: [u8; 9] = [240, 0, 32, 41, 2, 12, 0, 1, 247];
    colour_port.send(&msg).expect("Failed to send msg to LPX");
    let msg: [u8; 9] = [240, 0, 32, 41, 2, 12, 0, 127, 247];
    colour_port.send(&msg).expect("Failed to send msg to LPX");
    // Initialise the pad clours
    let make_colour = |section: &Section, colour: [u8; 3]| -> Vec<u8> {
        let mut colour_message: Vec<u8> = vec![240, 0, 32, 41, 2, 12, 3];
        let pads: Vec<u8> = section.pads();
        for pad in pads.iter() {
            colour_message.push(3);
            colour_message.push(*pad);
            colour_message.extend(colour.to_vec());
        }
        colour_message.push(247);
        colour_message
    };
    for section in sections.iter() {
        colour_port
            .send(&make_colour(section, section.main_colour))
            .unwrap();
    }
    loop {
        let message: [u8; 3] = match rx.recv() {
            Ok(m) => m,
            Err(err) => panic!("{}", err),
        };
        //eprintln!("loop message:{message:?}");
        if message[0] == 144 {
            // All MIDI notes from LPX start with 144, for initial noteon and noteoff
            let _velocity = message[2];

            // Find the section the pad is in
            let pad: u8 = message[1];
            for section in sections.iter() {
                if section.pad_in(pad) {
                    // got the section for a pad

                    // Send out the note
                    let velocity = message[2];
                    let message: [u8; 3] = [message[0], section.midi_note, velocity];
                    midi_note_out_port.send(&message)?;

                    if velocity > 0 {
                        // Note on
                        // Set colour of section to "active_colour"
                        let active_colour = make_colour(section, section.active_colour);

                        colour_port.send(&active_colour).unwrap();
                    } else {
                        // Not off
                        // Restore the colour
                        let main_colour = make_colour(section, section.main_colour);

                        colour_port.send(&main_colour).unwrap();
                    }
                }
            }
        } else if message[0] == 176 {
            // A control signal
            midi_ctl_out_port.send(&message).unwrap();
        }
    }
    // Ok(())
}
