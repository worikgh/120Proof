/// Blank the screen of the LPX. No arguments
use midi_connection::MIDICommunicator;

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
	// Comunication with LPX
	let mut midi_communicator1 = MIDICommunicator::new(
		"Launchpad X:Launchpad X MIDI 1",
		"120-Proof-1",
		|_, _, _| {},
		(),
		2,
	)?;

	let msg: [u8; 8 + 64 * 5] = [
		240, 0, 32, 41, 2, 12, 3, // pad, red, green, blue,
		3, 11, 0, 0, 0, // pad
		3, 12, 0, 0, 0, // pad
		3, 13, 0, 0, 0, // pad
		3, 14, 0, 0, 0, // pad
		3, 15, 0, 0, 0, // pad
		3, 16, 0, 0, 0, // pad
		3, 17, 0, 0, 0, // pad
		3, 18, 0, 0, 0, // pad
		3, 21, 0, 0, 0, // pad
		3, 22, 0, 0, 0, // pad
		3, 23, 0, 0, 0, // pad
		3, 24, 0, 0, 0, // pad
		3, 25, 0, 0, 0, // pad
		3, 26, 0, 0, 0, // pad
		3, 27, 0, 0, 0, // pad
		3, 28, 0, 0, 0, // pad
		3, 31, 0, 0, 0, // pad
		3, 32, 0, 0, 0, // pad
		3, 33, 0, 0, 0, // pad
		3, 34, 0, 0, 0, // pad
		3, 35, 0, 0, 0, // pad
		3, 36, 0, 0, 0, // pad
		3, 37, 0, 0, 0, // pad
		3, 38, 0, 0, 0, // pad
		3, 41, 0, 0, 0, // pad
		3, 42, 0, 0, 0, // pad
		3, 43, 0, 0, 0, // pad
		3, 44, 0, 0, 0, // pad
		3, 45, 0, 0, 0, // pad
		3, 46, 0, 0, 0, // pad
		3, 47, 0, 0, 0, // pad
		3, 48, 0, 0, 0, // pad
		3, 51, 0, 0, 0, // pad
		3, 52, 0, 0, 0, // pad
		3, 53, 0, 0, 0, // pad
		3, 54, 0, 0, 0, // pad
		3, 55, 0, 0, 0, // pad
		3, 56, 0, 0, 0, // pad
		3, 57, 0, 0, 0, // pad
		3, 58, 0, 0, 0, // pad
		3, 61, 0, 0, 0, // pad
		3, 62, 0, 0, 0, // pad
		3, 63, 0, 0, 0, // pad
		3, 64, 0, 0, 0, // pad
		3, 65, 0, 0, 0, // pad
		3, 66, 0, 0, 0, // pad
		3, 67, 0, 0, 0, // pad
		3, 68, 0, 0, 0, // pad
		3, 71, 0, 0, 0, // pad
		3, 72, 0, 0, 0, // pad
		3, 73, 0, 0, 0, // pad
		3, 74, 0, 0, 0, // pad
		3, 75, 0, 0, 0, // pad
		3, 76, 0, 0, 0, // pad
		3, 77, 0, 0, 0, // pad
		3, 78, 0, 0, 0, // pad
		3, 81, 0, 0, 0, // pad
		3, 82, 0, 0, 0, // pad
		3, 83, 0, 0, 0, // pad
		3, 84, 0, 0, 0, // pad
		3, 85, 0, 0, 0, // pad
		3, 86, 0, 0, 0, // pad
		3, 87, 0, 0, 0, // pad
		3, 88, 0, 0, 0, // pad
		247,
	];

	midi_communicator1.send(&msg)?;

	// let mut input: String = String::new();
	// input.clear();
	// stdin().read_line(&mut input)?; // wait for next enter key press
	Ok(())
}
