use midi_connection::MIDICommunicator;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut midi_communicator1 = MIDICommunicator::new(
        "Launchpad X:Launchpad X MIDI 1",
        "120-Proof",
        |_, _, _| {},
        (),
    )?;
    Ok(())
}
