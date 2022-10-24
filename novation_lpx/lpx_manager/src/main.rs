use midi_connection::MIDICommunicator;
use std::env;
use std::fs::File;
//use std::io::stdin;
use std::io::{self, BufRead};
use std::thread;
use std::time::Duration;
//use std::path::Path;

//use std::env;
// use midir;
use std::error::Error;

struct Adapter {
    // Adapter changes the MIDI note and sends it to the synthesiser
    // and sends colour change messages to the LPX
    midi_out_synth: MIDICommunicator<()>,
    midi_out_lpx: MIDICommunicator<()>,
    midi_map: [u8; 99], // key is MIDI from LPX value MIDI to synth
    scale: Vec<u8>,     // At most 12 unique intergers in 1..12 inclusive
    midi_note_to_pads: Vec<(Option<u8>, Option<u8>)>, // Each note at most 2 pads
    root_note: u8,
}
impl std::fmt::Debug for Adapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Adaptor")
    }
}
impl Adapter {
    fn adapt(&self, inp: u8) -> u8 {
        self.midi_map[inp as usize]
    }

    /// The colour of a pad.  Root notes get red(5), scale green(17),
    /// others cream(113)
    fn pad_colour(&self, pad_in: u8) -> Option<u8> {
        if pad_in % 10 > 0 && pad_in % 10 < 9 {
            let pad_out = self.adapt(pad_in);

            let diff_12 = ((self.root_note as i16 - pad_out as i16).abs() % 12) as u8;

            let note = if pad_out >= self.root_note {
                ((pad_out as i16 - self.root_note as i16).abs() % 12) as u8 + 1
            } else {
                12_u8 - if diff_12 == 0 { 12_u8 } else { diff_12 } + 1
            };
            let colour = match note {
                1 => 5, // Root note
                a => match self.scale.iter().find(|&&x| x == a) {
                    Some(_) => 17, // Scale note
                    None => 113,
                },
            };
            Some(colour)
        } else {
            // Not a MIDI key
            None
        }
    }

    fn new(
        midi_out_synth: MIDICommunicator<()>,
        midi_out_lpx: MIDICommunicator<()>,
        scale: &Vec<u8>,
        root_note: u8, // Where the scale is rooted.  The MIDI note
    ) -> Self {
        let mut midi_map = [0_u8; 99];

        // Each `delta[n]` is a mapping from a pad on LPX to a MIDI
        // signal.
        let delta: [u8; 80] = [
            1, 2, 3, 4, 5, 6, 7, 8, 0, 0, //
            6, 7, 8, 9, 10, 11, 12, 13, 0, 0, //
            11, 12, 13, 14, 15, 16, 17, 18, 0, 0, //
            16, 17, 18, 19, 20, 21, 22, 23, 0, 0, //
            21, 22, 23, 24, 25, 26, 27, 28, 0, 0, //
            26, 27, 28, 29, 30, 31, 32, 33, 0, 0, //
            31, 32, 33, 34, 35, 36, 37, 38, 0, 0, //
            36, 37, 38, 39, 40, 41, 42, 43, 0, 0,
        ];

        // `delta[n]` + `p` is a midi signal.  When a pad `n` is pressed
        // the MIDI signal is `delta[n] + p`.
        let p = 10;

        let mut midi_note_to_pads = (0..99)
            .map(|_| (None, None))
            .collect::<Vec<(Option<u8>, Option<u8>)>>();
        // The middle key in this scheme is 34. So adjust the note to
        // make it proportional for `root_note`.
        let adj_note = root_note - 34;
        let mut i: u8 = 11;
        for d in delta {
            if i % 10 != 0 && i % 10 != 9 {
                // `i` is the number for a pad.  No pads 10,
                // 20,... and pads 19, 29,... are control pads
                let midi_note = d + p + adj_note;

                // Incoming MIDI `i` becomes `pad`.  E.g. MIDI == 32
                // print!("pad({}) i({}) ", pad, i);
                midi_map[i as usize] = midi_note;
                let row = i / 10;
                let col = i % 10;
                // This function returns the (at most) two pads that
                // emit this note
                let f = |p| {
                    if p < 80 && col > 5 && col < 9 {
                        // Not in top row, and in right hand
                        // three columns.  Pads here have a
                        // duplicate in the row above
                        Some((row + 1) * 10 + col - 6)
                    } else if p > 20 && col < 4 {
                        // Not in bottom row and in left hand columns.
                        // Pads here have
                        // a duplicate in the row below
                        Some((row - 1) * 10 + col + 5)
                    } else {
                        None
                    }
                }; //f

                let pads: (Option<u8>, Option<u8>) = (Some(i), f(i));

                midi_note_to_pads[midi_note as usize] = pads;
            }
            i += 1;
        }

        Self {
            midi_out_synth: midi_out_synth,
            midi_out_lpx: midi_out_lpx,
            midi_map: midi_map,
            scale: scale.to_vec(),
            midi_note_to_pads: midi_note_to_pads,
            root_note: root_note,
        }
    }
}

/// The names of MIDI devices set up in this.  
struct DeviceNames {
    /// The MIDI notes from the LPX.  The source
    midi_source_lpx: String,
    /// The MIDI notes from the LPX.  This end
    midi_source_lpx_120: String,

    /// Send MIDI commands to control pad colour on LPX
    midi_sink_lpx: String,
    midi_sink_lpx_120: String,

    midi_sink_synth: String,
    midi_sink_synth_120: String,
}

impl DeviceNames {
    fn new(cfg_fn: &str) -> io::Result<DeviceNames> {
        // Read a configuration file for midi_source_lpx, midi_sink_lpx, midi_sink_synth
        let mut midi_source_lpx = "".to_string(); //"Launchpad X:Launchpad X MIDI 2".to_string();
        let mut midi_sink_lpx = "".to_string();
        let mut midi_sink_synth = "".to_string();

        let file = File::open(cfg_fn)?;
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(l) = line {
                // `l` is the line
                if l.starts_with("midi_source_lpx:") {
                    midi_source_lpx = l.strip_prefix("midi_source_lpx:").unwrap().to_string();
                } else if l.starts_with("midi_sink_lpx:") {
                    midi_sink_lpx = l.strip_prefix("midi_sink_lpx:").unwrap().to_string();
                } else if l.starts_with("midi_sink_synth:") {
                    midi_sink_synth = l.strip_prefix("midi_sink_synth:").unwrap().to_string();
                } else {
                    panic!("{} misunderstood", &l);
                }
            }
        }
        Ok(DeviceNames {
            midi_source_lpx: midi_source_lpx, //"Launchpad X:Launchpad X MIDI 2",
            midi_source_lpx_120: "120-Proof-MIDI-In-LPX".to_string(),

            midi_sink_lpx: midi_sink_lpx, //"Launchpad X:Launchpad X MIDI 1".to_string(),
            midi_sink_lpx_120: "120-Proof-MIDI-Out-LPX".to_string(),

            midi_sink_synth: midi_sink_synth, //"Pure Data:Pure Data Midi-In 2".to_string(),
            midi_sink_synth_120: "120-Proof-MIDI-Out-PD".to_string(),
        })
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // This is the scale.  Should be able to pass this in on the command line.
    let scale: Vec<u8>;
    let root_note: u8;
    if args.is_empty() {
        panic!("Need arguments");
    }
    // First argument is the config file name.  Next the root
    // note.  The rest of the arguments are the scale
    let mut iter = args.iter();
    let cfg_fn = iter.nth(1).unwrap().as_str();
    let root_note_iv = iter.nth(0).unwrap().as_str();

    root_note = root_note_iv.parse::<u8>()?;

    let mut intermediate_value: Vec<u8> = Vec::new();
    for s in iter {
        // s is &String
        match s.as_str().parse() {
            Ok(value) => intermediate_value.push(value),
            Err(err) => panic!("s({}) err({:?})", s, err),
        };
    }
    scale = intermediate_value;

    let device_names = match DeviceNames::new(cfg_fn) {
        Ok(dn) => dn,
        Err(err) => panic!("Cannot make DeviceNames from {}. Err({})", cfg_fn, err),
    };

    let midi_out_synth: MIDICommunicator<()> = MIDICommunicator::new(
        device_names.midi_sink_synth.as_str(),
        device_names.midi_sink_synth_120.as_str(),
        |_, _, _| {},
        (),
        2,
    )?;

    let midi_out_lpx: MIDICommunicator<()> = MIDICommunicator::new(
        device_names.midi_sink_lpx.as_str(),
        device_names.midi_sink_lpx_120.as_str(),
        |_, _, _| {},
        (),
        2,
    )?;

    let mut adapter = Adapter::new(midi_out_synth, midi_out_lpx, &scale, root_note);
    // Initialise LPX colours
    for i in 11..90 {
        if i % 10 > 0 && i % 10 < 9 {
            // let ten_millis = Duration::from_millis(100);
            // thread::sleep(ten_millis);

            let colour = adapter.pad_colour(i as u8).unwrap();
            let out_message_colour_change: [u8; 11] = [240, 0, 32, 41, 2, 12, 3, 0, i, colour, 247];

            match adapter.midi_out_lpx.send(&out_message_colour_change) {
                Ok(()) => {}
                Err(err) => eprintln!("Initialising colours: Failed send: {:?}", err),
            };
        }
    }

    // The process that listens

    let _midi_in: MIDICommunicator<Adapter> = MIDICommunicator::new(
        device_names.midi_source_lpx.as_str(),
        device_names.midi_source_lpx_120.as_str(),
        |_stamp, message, adapter| {
            let pad_in = message[1];
            let velocity = message[2];

            // Send note to synthesiser
            match message[0] {
                144 => {
                    if pad_in % 10 > 0 && pad_in % 10 < 9 {
                        // A key press, adapt it (translate the position
                        // on the LPX represented by `pad_in` into a MIDI
                        // note)
                        let midi_note_out: u8 = adapter.adapt(pad_in);
                        let out_message_midi_note = [144, midi_note_out, velocity];
                        match adapter.midi_out_synth.send(&out_message_midi_note) {
                            Ok(()) => (),
                            Err(err) => eprintln!("Sending note: Failed send: {:?}", err),
                        };

                        // The key that is pressed, flash it blue(50) as it is
                        // pressed.  It's standard colour otherwise
                        let pad_colour: u8 = match velocity {
                            0 =>
                            // Key up.  Return to unpressed colour
                            {
                                adapter.pad_colour(pad_in).unwrap() // Safe as pad_in is filtered
                            }
                            _ => 50,
                        };

                        // There are possibly two pads to adjust colour of
                        let pads = adapter.midi_note_to_pads[midi_note_out as usize];
                        if let Some(p) = pads.0 {
                            let out_message_colour_change: [u8; 11] =
                                [240, 0, 32, 41, 2, 12, 3, 0, p, pad_colour, 247];
                            match adapter.midi_out_lpx.send(&out_message_colour_change) {
                                Ok(()) => (),
                                Err(err) => {
                                    eprintln!("Press colour change: Failed send: {:?}", err)
                                }
                            };
                        }
                        if let Some(p) = pads.1 {
                            let out_message_colour_change: [u8; 11] =
                                [240, 0, 32, 41, 2, 12, 3, 0, p, pad_colour, 247];
                            match adapter.midi_out_lpx.send(&out_message_colour_change) {
                                Ok(()) => (),
                                Err(err) => {
                                    eprintln!("Press colour change(2) : Failed send: {:?}", err)
                                }
                            };
                        }
                    }
                }
                _ => match adapter.midi_out_lpx.send(&[message[0], pad_in, velocity]) {
                    Ok(()) => (),
                    Err(err) => eprintln!("Random message(?): Failed send: {:?}", err),
                },
            };
        },
        adapter,
        1,
    )?;

    // let mut input: String = String::new();
    // input.clear();
    // stdin().read_line(&mut input)?; // wait for next enter key press
    loop {
        thread::sleep(Duration::from_millis(100));
    }
    // Ok(())
}
