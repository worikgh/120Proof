use midi_connection::MIDICommunicator;
use std::env;
use std::io::stdin;

//use std::env;
use midir;
use std::error::Error;

struct Adapter {
    midi_out: MIDICommunicator<()>,
    midi_map: [u8; 99],
}
impl std::fmt::Debug for Adapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Adaptor")
    }
}
impl Adapter {
    fn adapt(&self, inp: u8) -> u8 {
        // println!("midi_map({:?})", self.midi_map);
        // println!("Adapter::adapt({}) -> {}", inp, self.midi_map[inp as usize]);
        self.midi_map[inp as usize]
    }
    fn new(midi_out: MIDICommunicator<()>) -> Self {
        let mut midi_map = [0_u8; 99];

        // `delta` + `p` is a midi signal
        let p = 10;
        let delta: [u8; 80] = [
            1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 6, 7, 8, 9, 10, 11, 12, 13, 0, 0, 11, 12, 13, 14, 15, 16,
            17, 18, 0, 0, 16, 17, 18, 19, 20, 21, 22, 23, 0, 0, 21, 22, 23, 24, 25, 26, 27, 28, 0,
            0, 26, 27, 28, 29, 30, 31, 32, 33, 0, 0, 31, 32, 33, 34, 35, 36, 37, 38, 0, 0, 36, 37,
            38, 39, 40, 41, 42, 43, 0, 0,
        ];
        // The middle key in this scheeme is 34.  Middle C is MIDI 60
        // So adjustment...
        let adj_mid_c = 60 - 34;
        let mut i = 11;
        for d in delta {
            if i > 0 {
                let pad = d + p + adj_mid_c;
                // Incomming MIDI `i` becomes `pad`.  E.g. MIDI == 32
                // print!("pad({}) i({}) ", pad, i);
                midi_map[i] = pad;
            }
            i += 1;
        }
        println!();
        Self {
            midi_out: midi_out,
            midi_map: midi_map,
        }
    }
}

fn find_port<T>(midi_io: &T, device: &str) -> Option<T::Port>
where
    T: midir::MidiIO,
{
    let mut device_port: Option<T::Port> = None;
    for port in midi_io.ports() {
        if let Ok(port_name) = midi_io.port_name(&port) {
            println!("Port: {}", &port_name);
            if port_name.contains(device) {
                device_port = Some(port);
                break;
            }
        }
    }
    device_port
}

fn main() -> Result<(), Box<dyn Error>> {
    // let midi_out = midir::MidiOutput::new("120-Proof")?;
    let args: Vec<_> = env::args().collect();
    let midi_out: MIDICommunicator<()> = MIDICommunicator::new(
        "Pure Data:Pure Data Midi-In 1",
        "120-Proof",
        |_, _, _| {},
        (),
        2,
    )?;

    let midi_input = midir::MidiInput::new("MIDITest").unwrap();

    let _midi_in: MIDICommunicator<Adapter> = MIDICommunicator::new(
        "Launchpad X:Launchpad X MIDI 2",
        "120-Proof-2",
        |stamp, message, adapter| {
            eprintln!("midi_in stamp({:?}) message({:?})", &stamp, &message);
            let out_message = match message[0] {
                144 => {
                    // A key press
                    [144, adapter.adapt(message[1]), message[2]]
                }
                _ => [message[0], message[1], message[2]],
            };
            println!("out_message({:?})", &out_message);
            adapter.midi_out.send(&out_message).unwrap()
            // adapter.midi_out.send(message)?
        },
        Adapter::new(midi_out),
        1,
    )?;

    let mut input: String = String::new();
    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press
    Ok(())
}
