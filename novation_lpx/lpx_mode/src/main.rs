/// Sets the mode of the LPX
/// See page seven of the LPX Programmers Reference
/// 00h (0): Session (only selectable in DAW mode)
/// 01h (1): Note mode
/// 04h (4): Custom mode 1 (Drum Rack by factory default)
/// 05h (5): Custom mode 2 (Keys by factory default)
/// 06h (6): Custom mode 3 (Lighting mode in Drum Rack layout by factory default)
/// 07h (7): Custom mode 4 (Lighting mode in Session layout by factory default)
/// 0Dh (13): DAW Faders (only selectable in DAW mode) 7Fh (127): Programmer mode
use midi_connection::MIDICommunicator;

use std::env;
use std::error::Error;
// use std::thread;
// use std::time;
fn main() -> Result<(), Box<dyn Error>> {
    let mut midi_communicator1 = MIDICommunicator::new(
        "Launchpad X:Launchpad X MIDI 1",
        "120-Proof-1",
        |_, _, _| {},
        (),
        3,
    )?;
    // This is the MIDI message that puts the LPX into programmer's
    // mode.

    // Mode is the first and only argument
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.len() > 2 {
        // No args or too many args
        println!("Usage:\n\t{} <mode>\n<mode> in: \n 00h (0): Session (only selectable in DAW mode)\n 01h (1): Note mode\n 04h (4): Custom mode 1 (Drum Rack by factory default)\n 05h (5): Custom mode 2 (Keys by factory default)\n 06h (6): Custom mode 3 (Lighting mode in Drum Rack layout by factory default)
\n 07h (7): Custom mode 4 (Lighting mode in Session layout by factory default)\n 0Dh (13): DAW Faders (only selectable in DAW mode) 7Fh (127): Programmer mode\n", args[0]);
    } else {
        assert!(args.len() == 2);
        let mode: &str = &args[1];
        match mode.parse() {
            Ok(mode) => {
                let msg: [u8; 9] = [240, 0, 32, 41, 2, 12, 0, mode, 247];
                midi_communicator1.send(&msg).unwrap()
            },
            Err(err) => eprintln!("Mode {}: {:?}", mode, err),
        };
    }
    // Get the replies.  In midi_communicator there is an infinite
    // loop receiving and printing MIDI messages
    // loop {
    //     println!("tick/tock");
    //     thread::sleep(time::Duration::from_secs(60))
    // }
    Ok(())
}
