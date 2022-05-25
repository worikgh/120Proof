use midi_connection::MIDICommunicator;

use std::env;
use std::error::Error;
// use std::io::stdin;
fn main() -> Result<(), Box<dyn Error>> {
    // Get the pad (11..99) and colour (r,g,b)
    let args: Vec<String> = env::args().collect();

    let mut midi_communicator1 = MIDICommunicator::new(
        "Launchpad X:Launchpad X MIDI 1",
        "120-Proof-1",
        |_, _, _| {},
        (),
        2,
    )?;

    let pad: u8 = args[1].parse()?;
    let red: u8 = args[2].parse()?;
    let green: u8 = args[3].parse()?;
    let blue: u8 = args[4].parse()?;
    let msg: [u8; 13] = [240, 0, 32, 41, 2, 12, 3, 3, pad, red, green, blue, 247];

    midi_communicator1.send(&msg)?;

    // let mut input: String = String::new();
    // input.clear();
    // stdin().read_line(&mut input)?; // wait for next enter key press
    Ok(())
}
