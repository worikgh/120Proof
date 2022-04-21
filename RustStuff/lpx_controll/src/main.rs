extern crate midir;
use std::collections::HashMap;

// Use the MIDI control keys from the LPX to run programes.  


use std::error::Error;
use std::io::{stdin};

use midir::{Ignore, MidiInput, MidiInputPort};

// Dispatcher matches a control key to an executable and executes it
struct Dispatcher {
    // Associate a control value with a command
    table: HashMap<u8, String>,    
}

impl Dispatcher {
    fn new() -> Self {
	let mut table:HashMap<u8, String> = HashMap::new();
	table.insert(19, "CTL.19".to_string());
	table.insert(29, "CTL.29".to_string());
	table.insert(39, "CTL.39".to_string());
	table.insert(49, "CTL.49".to_string());
	table.insert(59, "CTL.59".to_string());
	table.insert(69, "CTL.69".to_string());
	table.insert(79, "CTL.79".to_string());
	table.insert(89, "CTL.89".to_string());
	Self {
	    table:table,
	}
    }
    fn run(&self, ctl:u8){
	match self.table.get(&ctl) {
	    Some(cmd) => println!("Run: {}", &cmd),
	    None => (),
	}
    }
}


fn main() {
    let dispatcher = Dispatcher::new();
    match run(dispatcher) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn process_message(message:&[u8;3], dispatcher: &Dispatcher){
    if message[0] == 176 {
	// A ctl message
	
	let key = message[1];
	let vel = message[2];
	if key >= 19 {
	    // There is some noise coming from the LPX with ctl-key 7
	    // The rest are control signals tyhat we want
	    if vel > 0 {
		// 0 VEL is pad release
		dispatcher.run(key);
	    }
	}
    }
}
	    

fn run(dispatcher:Dispatcher) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("120 Proof")?;
    midi_in.ignore(Ignore::None);

    // The port we get messages on
    let mut port: Option<MidiInputPort> = None;
    let source_port = "Launchpad X:Launchpad X MIDI 2".to_string().into_bytes();
    
    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    for (i, p) in in_ports.iter().enumerate() {
        let port_name = midi_in.port_name(p)?.into_bytes();
        let mut accept: bool = true;
        for i in 0..port_name.len() {
            if i < source_port.len() && source_port[i] != port_name[i] {
                accept = false;
                break;
            }
	}
        if accept {
	    let p = midi_in
                .ports()
                .get(i)
                .ok_or("Invalid port number")?
                .clone();
	    port = Some(p);
	    break;
        }
    }
    let in_port = port.unwrap();

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |_stamp, message, dispatcher| {
            // println!(
            //     "{}: Msg: {:?} (len = {})",
            //     (stamp as f64) / 1_000_000.0,
            //     &message,
            //     message.len()
            // );
	    if message.len() == 3 {
		let array = <[u8; 3]>::try_from(message).unwrap();
		process_message(&array, dispatcher);
	    }
        },
        dispatcher,
    )?;

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}

