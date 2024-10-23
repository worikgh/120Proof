use crate::jack_connections::JackConnections;
use crate::midi::Midi;
use crate::pedals_available::get_pipes_from_file;
use std::env::args;
use std::error::Error;
use std::thread;
use std::time::Duration;
mod jack_connections;
mod midi;
mod pedals_available;
use crate::midi::MidiData;

// use subprocess::Communicator;
// use subprocess::Popen;
// use subprocess::PopenConfig;
fn main() -> Result<(), Box<dyn Error>> {
    let card: String = args().nth(1).unwrap();
    run(card).unwrap();
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
fn handle_midi(b: &[u8], connection_cache: &mut MidiData) {
    match handle_midi_real(b, connection_cache) {
        Ok(_) => (),
        Err(err) => eprintln!("Cannot handle midi: {err}"),
    };
    // println!("handle_midi {:?}", b);
}
fn handle_midi_real(
    b: &[u8],
    midi_data: &mut MidiData,
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    eprintln!("handle_midi_real {:?}", b);
    let mut jack_connetions = JackConnections::new("client_name");
    let a = b[1];
    if a == midi_data.last {
        eprintln!("Idempotent!");
        return Ok(());
    }
    let selected_pedal: &str = match a {
        0 => "A",
        1 => "B",
        2 => "C",
        3 => "D",
        _ => panic!("Trying to handle {a}"),
    };
    midi_data.last = a;

    let pipes: Vec<(String, String)> = get_pipes_from_file(selected_pedal)?;
    for pipe in pipes.iter() {
        match jack_connetions.make_connection(pipe.0.clone(), pipe.1.clone()) {
            Ok(_) => println!("Connected"),
            Err(err) => println!("Failed: {err}"),
        };
    }
    if !midi_data.connection_cache.is_empty() {
        for pipe in &mut *midi_data.connection_cache {
            let src = pipe.0.clone();
            let dst = pipe.1.clone();
            match jack_connetions.unmake_connection(src.clone(), dst.clone()) {
                Ok(_) => println!("Disconnected {src} {dst}"),
                Err(err) => println!("Failed: {err} Disconnect  {src} {dst}"),
            };
        }
    }
    midi_data.connection_cache = pipes;
    println!("Pedal: {selected_pedal}");
    Ok(())
}

fn run(name: String) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let midi = Midi::new(name)?;
    midi.run(handle_midi)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
