use crate::section::Section;
use midi_connection::MIDICommunicator;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
use std::sync::mpsc::{self, Receiver, Sender};
mod lpx_drum_error;
mod section;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Pass name of drum definition, JSON formatted, file as sole argument");
    }
    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let result: Vec<Section> = Section::parse_json(&content)?;
    if let Err(err) = Section::check_sections(&result) {
        panic!("{err}");
    }
    // xMidi to and from  the LPX

    // The closure in the MIDICommunicator uses a channel to send all
    // MIDI messages that are for this programme back here
    let (tx, rx): (Sender<[u8; 3]>, Receiver<[u8; 3]>) = mpsc::channel::<[u8; 3]>();

    let midi_lpx: MIDICommunicator<Sender<[u8; 3]>> = MIDICommunicator::new(
        "Launchpad X:Launchpad X LPX MIDI In",
        "120-Proof-LpxDrums",
        move |_stamp, message, sender| {
            // The messages that wil be processed here are length
            // three.  MIDI notes are also length three, and when they
            // come by the controls are inactivated for a period to
            // avoid accedentally changing the set up of the
            // instrument
            eprintln!("MIDICommunicator closure.  msg: {:?}", message);
            if message.len() == 3 {
                let message: [u8; 3] = message.try_into().unwrap();
                sender.send(message).unwrap();
            }
        },
        tx.clone(),
        3,
    )
    .unwrap();

    loop {
        let message: [u8; 3] = match rx.recv() {
            Ok(m) => m,
            Err(err) => panic!("{}", err),
        };
        println!("In main loop: {message:?}");
    }
    //    Ok(())
}
