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
        0, 11, 0, // pad
        0, 12, 0, // pad
        0, 13, 0, // pad
        0, 14, 0, // pad
        0, 15, 0, // pad
        0, 16, 0, // pad
        0, 17, 0, // pad
        0, 18, 0, // pad
        0, 21, 0, // pad
        0, 22, 0, // pad
        0, 23, 0, // pad
        0, 24, 0, // pad
        0, 25, 0, // pad
        0, 26, 0, // pad
        0, 27, 0, // pad
        0, 28, 0, // pad
        0, 31, 0, // pad
        0, 32, 0, // pad
        0, 33, 0, // pad
        0, 34, 0, // pad
        0, 35, 0, // pad
        0, 36, 0, // pad
        0, 37, 0, // pad
        0, 38, 0, // pad
        0, 41, 0, // pad
        0, 42, 0, // pad
        0, 43, 0, // pad
        0, 44, 0, // pad
        0, 45, 0, // pad
        0, 46, 0, // pad
        0, 47, 0, // pad
        0, 48, 0, // pad
        0, 51, 0, // pad
        0, 52, 0, // pad
        0, 53, 0, // pad
        0, 54, 0, // pad
        0, 55, 0, // pad
        0, 56, 0, // pad
        0, 57, 0, // pad
        0, 58, 0, // pad
        0, 61, 0, // pad
        0, 62, 0, // pad
        0, 63, 0, // pad
        0, 64, 0, // pad
        0, 65, 0, // pad
        0, 66, 0, // pad
        0, 67, 0, // pad
        0, 68, 0, // pad
        0, 71, 0, // pad
        0, 72, 0, // pad
        0, 73, 0, // pad
        0, 74, 0, // pad
        0, 75, 0, // pad
        0, 76, 0, // pad
        0, 77, 0, // pad
        0, 78, 0, // pad
        0, 81, 0, // pad
        0, 82, 0, // pad
        0, 83, 0, // pad
        0, 84, 0, // pad
        0, 85, 0, // pad
        0, 86, 0, // pad
        0, 87, 0, // pad
        0, 88, 0, // pad
        247,
    ];

    midi_communicator1.send(&msg)?;

    // let mut input: String = String::new();
    // input.clear();
    // stdin().read_line(&mut input)?; // wait for next enter key press
    Ok(())
}
