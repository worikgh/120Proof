/// Blank the screen of the LPX. No arguments
use midi_connection::MIDICommunicator;

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    // Comunication with LPX
    let mut midi_communicator1 = MIDICommunicator::new(
        "Launchpad X:Launchpad X LPX MIDI In",
        "120-Proof-1",
        |_, _, _| {},
        (),
        2,
    )?;
    // 8 bytes overhead, 64 pads, 3 bytes per pad
    let msg: [u8; 8 + 64 * 3] = [
        240, 0, 32, 41, 2, 12, 3,
        // Pad colour data:
        // 0 => Colour is from palette
        // 11-88 Pad number
        // 0 => black
        0, 11, 0, // first pad
        0, 12, 0, 
        0, 13, 0, 
        0, 14, 0, 
        0, 15, 0, 
        0, 16, 0, 
        0, 17, 0, 
        0, 18, 0, 
        0, 21, 0, 
        0, 22, 0, 
        0, 23, 0, 
        0, 24, 0, 
        0, 25, 0, 
        0, 26, 0, 
        0, 27, 0, 
        0, 28, 0, 
        0, 31, 0, 
        0, 32, 0, 
        0, 33, 0, 
        0, 34, 0, 
        0, 35, 0, 
        0, 36, 0, 
        0, 37, 0, 
        0, 38, 0, 
        0, 41, 0, 
        0, 42, 0, 
        0, 43, 0, 
        0, 44, 0, 
        0, 45, 0, 
        0, 46, 0, 
        0, 47, 0, 
        0, 48, 0, 
        0, 51, 0, 
        0, 52, 0, 
        0, 53, 0, 
        0, 54, 0, 
        0, 55, 0, 
        0, 56, 0, 
        0, 57, 0, 
        0, 58, 0, 
        0, 61, 0, 
        0, 62, 0, 
        0, 63, 0, 
        0, 64, 0, 
        0, 65, 0, 
        0, 66, 0, 
        0, 67, 0, 
        0, 68, 0, 
        0, 71, 0, 
        0, 72, 0, 
        0, 73, 0, 
        0, 74, 0, 
        0, 75, 0, 
        0, 76, 0, 
        0, 77, 0, 
        0, 78, 0, 
        0, 81, 0, 
        0, 82, 0, 
        0, 83, 0, 
        0, 84, 0, 
        0, 85, 0, 
        0, 86, 0, 
        0, 87, 0, 
        0, 88, 0, // last pad
        247,
    ];

    midi_communicator1.send(&msg)?;

    // let mut input: String = String::new();
    // input.clear();
    // stdin().read_line(&mut input)?; // wait for next enter key press
    Ok(())
}
