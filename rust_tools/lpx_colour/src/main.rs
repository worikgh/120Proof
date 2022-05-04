use midi_connection::MIDICommunicator;

use std::env;
use std::error::Error;
fn get_colour(args: &Vec<String>) -> Result<(u8, u8, u8), Box<dyn Error>> {
    let red: u8 = args[1].parse()?;
    let green: u8 = args[2].parse()?;
    let blue: u8 = args[3].parse()?;
    Ok((red, green, blue))
}
fn main() -> Result<(), Box<dyn Error>> {
    // Get the pad (11..99) and colour (r,g,b)
    let args: Vec<String> = env::args().collect();
    match args[1].parse::<u8>() {
        Err(err) => eprintln!("{:?}", err),
        Ok(pad) => {
            match get_colour(&args) {
                Err(err) => eprintln!("{:?}", err),
                Ok((red, green, blue)) => {
                    let mut midi_communicator1 =
                        MIDICommunicator::new("Launchpad X:Launchpad X MIDI 1")?;
                    let msg: [u8; 13] = [240, 0, 32, 41, 2, 12, 3, 3, pad, red, green, blue, 247];
                    // let msg: [u8; 12] = [240, 0, 32, 41, 2, 12, 3, pad, red, green, blue, 247];
                    midi_communicator1.send(&msg).unwrap()
                }
            };
        }
    };
    Ok(())
}
