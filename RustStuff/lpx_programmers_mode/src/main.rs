#![allow(non_snake_case)]
extern crate midir;

use std::error::Error;
// use std::io::{stdin, stdout, Write};

use midir::{MidiOutput, MidiOutputPort};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {

    // This is the MIDI message that puts the LPX into programmer's
    // mode.
    let msg: [u8; 9] = [240, 0, 32, 41, 2, 12, 0, 127, 247];

    // The port that we will send the message to
    let mut port: Option<MidiOutputPort> = None;
    let source_port = "Launchpad X:Launchpad X MIDI 1".to_string().into_bytes();

    // Get the port by name
    let midi_out = MidiOutput::new("120 Proof")?;
    for (i, p) in midi_out.ports().iter().enumerate() {
        let port_name = midi_out.port_name(p)?.into_bytes();
        let mut accept: bool = true;
        for i in 0..port_name.len() {
            if i < source_port.len() && source_port[i] != port_name[i] {
                accept = false;
                break;
            }
        }
        if accept {
            let p = midi_out
                .ports()
                .get(i)
                .ok_or("Invalid port number")?
                .clone();
            port = Some(p);
            break;
        }
    }
    let out_port = port.unwrap();
    let mut conn_out = midi_out.connect(&out_port, "120 Proof Connection")?;
    conn_out
        .send(&msg)
        .unwrap_or_else(|_| println!("Error when forwarding message ..."));
    Ok(())
}
