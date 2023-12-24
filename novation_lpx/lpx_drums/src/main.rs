//! Use a Novation LPX Pad as a musical instrument
//! Control the colours on the display
//! Translate the MIDI signals from the raw PAD number from the LPX into
//! noteon/noteoff signals
//! On start up connect directly to the LPX (it must exist ad be
//! available) then set up a virtual connection for the synthesiser
//! and connect to it later
//! ["Programmer's Manual" ](https://fael-downloads-prod.focusrite.com/customer/prod/s3fs-public/downloads/Launchpad%20X%20-%20Programmers%20Reference%20Manual.pdf)
extern crate midir;
mod lpx_drum_error;
mod section;

use crate::midir::os::unix::VirtualOutput;
use crate::section::Section;
use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};  
use std::env;
use std::error::Error;
use std::fs::File;
use std::result::Result;
use std::sync::mpsc::{self, Receiver, Sender};
use std::io::prelude::*;

/// Initialise a vector of `Section` from a file.
fn load_sections(filename: &str) -> Result<Vec<Section>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Creatre the sections from the file
    let result: Vec<Section> = Section::parse_json(&content)?;
    Ok(result)
}

// Get a MIDI port that has a name containing `keyword` 
fn get_midi_port<T: midir::MidiIO>(midi_io: &T, keyword: &str) -> Option<T::Port> {
    for port in midi_io.ports() {
        let name = match midi_io.port_name(&port) {
            Ok(name) => name,
            Err(_) => continue,
        };

        if name.contains(keyword) {
            eprintln!("Guessing port from keyword: {keyword} name: {name}");
            return Some(port);
        }
    }

    None
}

/// Create an output MIDI port to the LPX.
/// It uses the passed parameter `name` to create a prort: LpxDrums:<name>
fn get_midi_out(name: &str) -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_output = MidiOutput::new("LpxDrums")?;
    let port = get_midi_port(&midi_output, "Launchpad X LPX MIDI In").unwrap(); //.ok_or(Err("Failed guess port".into())?);
    Ok(midi_output.connect(&port, name)?)
}

/// Create a MIDI input port, connected from the LPX MIDI port.
/// `name` is the port name for the created port
/// `f` is the function that takes a channel and sends the MIDI that
/// it wants to handle down that channel
/// `tx` is the channel
fn get_midi_in(
    name: &str,
    f: impl FnMut(u64, &[u8], &mut Sender<[u8; 3]>) + Send + 'static,
    tx: Sender<[u8; 3]>,
) -> Result<MidiInputConnection<Sender<[u8; 3]>>, Box<dyn Error>> {
    let midi_input = MidiInput::new("LpxDrums")?;
    let port = get_midi_port(&midi_input, "Launchpad X LPX MIDI In").unwrap();
    //.ok_or(Err("Failed guess port".into())?);
    let result = midi_input.connect(&port, name, f, tx)?;
    Ok(result)
}


fn main() -> Result<(), Box<dyn Error>> {

    // The only argument is a configuration file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Pass name of drum definition, JSON formatted, file as sole argument");
    }
    let filename = &args[1];

    // Initialise the collection of `Section` from the file. (See `section.rs`)
    let sections: Vec<Section> = load_sections(filename)?;


    // The channel to send MIDI messages, received from the LPX in the
    // MidiInputConnection, here to the main thread
    let (tx, rx): (Sender<[u8; 3]>, Receiver<[u8; 3]>) =
	mpsc::channel::<[u8; 3]>();

    // Connect to the LPX to receive pad press events.  `f` is the
    // function that handles input MIDI and sends them back to themain
    // thread
    let f = move |_stamp, message: &[u8], tx: &mut Sender<[u8; 3]>| {
        // let message = MidiMessage::from_bytes(message.to_vec());
        if message.len() == 3 {
            let m3: [u8; 3] = message.try_into().unwrap();
            tx.send(m3).unwrap();
        }
    };
    // The port stays open as long as `_in` is in scope 
    let _in = get_midi_in("read_input", f, tx.clone())?;
    
    // Create an output port to the LPX for sending it colour.
    let mut colour_port: MidiOutputConnection = get_midi_out("colour_port")?;

    // Selecting Layouts (page 7 programmers manual).  127 => "Programmer Mode"
    let msg: [u8; 9] = [240, 0, 32, 41, 2, 12, 0, 127, 247];
    match colour_port.send(&msg) {
        Ok(()) => (),
        Err(err) => eprintln!("{err}: Failed to send msg to LPX: {msg:?}"),
    };

    let make_colour = |section: &Section, colour: [u8; 3]| -> Vec<u8> {
	// Buid the MIDI command that sets the colours of all the pads
	// in a section (they are all the same colour - part of what
	// defines a section).  One long MIDI sysex message that sets
	// many pads in one command
	
	// "LED lighting SysEx message" programmer's mabual page 15
        let mut colour_message: Vec<u8> = vec![240, 0, 32, 41, 2, 12, 3];
        let pads: Vec<u8> = section.pads();
        for pad in pads.iter() {
            colour_message.push(3); // RGB colour
            colour_message.push(*pad); // Pad index 
            colour_message.extend(colour.to_vec()); // RGB tripple
        }
        colour_message.push(247); // End message
        colour_message
    };

    // Initialise the colours
    for section in sections.iter() {
        let colour = make_colour(section, section.main_colour);
        match colour_port.send(&colour) {
            Ok(()) => (),
            Err(err) => eprintln!("{err}: Cannot send colour: {colour:?}"),
        };
    }

    // Establish the output that sends MIDI to whatever software will
    // interpret the MIDI to create sound and MIDI controls to
    // whatever interprets them.  An external programme will have to
    // conmplete these setups as this programme does not know what
    // they will be
    let midi_out: MidiOutput = MidiOutput::new("LpxDrumNote")?;
    let port_name = "port";
    let mut midi_note_out_port: MidiOutputConnection =
	midi_out.create_virtual(port_name)?;

    let midi_out: MidiOutput = MidiOutput::new("LpxDrumCtl")?;
    let port_name = "port";
    let mut midi_ctl_out_port: MidiOutputConnection =
	midi_out.create_virtual(port_name)?;
    eprintln!("2 Virtual MIDI Output port 'LpxDrumNote:{port_name}' is open");
    eprintln!("3 Virtual MIDI Output port 'LpxDrumCtl:{port_name}' is open");

    // Main loop.  
    loop {
        let message: [u8; 3] = match rx.recv() {
            Ok(m) => m,
            Err(err) => panic!("{}", err),
        };
        if message[0] == 144 {
            // All MIDI notes from LPX start with 144, for initial
            // noteon and noteoff
            let _velocity = message[2];

            // Find the section the pad is in
            let pad: u8 = message[1];
            for section in sections.iter() {
                if section.pad_in(pad) {
                    // got the section for a pad

                    // Send out the note
                    let velocity = message[2];
                    let message: [u8; 3] = [message[0],
					    section.midi_note,
					    velocity];
                    midi_note_out_port.send(&message)?;

                    if velocity > 0 {
                        // Note on
                        // Set colour of section to "active_colour"
                        let active_colour = make_colour(section,
							section.active_colour);
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
            eprintln!("control_port On: Message{message:?}");
            midi_ctl_out_port.send(&message).unwrap();
        }
    }
    // Ok(())
}
