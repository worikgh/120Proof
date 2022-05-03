// Use the MIDI control keys from the LPX to run programes.

extern crate midir;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::stdin;
use std::process;

use midir::{Ignore, MidiInput, MidiInputPort, MidiOutput, MidiOutputPort};

fn log(msg: &str) {
    println!("{}", msg);
}

// Dispatcher matches a control key to an executable and executes it
struct Dispatcher {
    // Associate a control value with a command
    up_table: HashMap<u8, String>,
    down_table: HashMap<u8, String>,
    last: Option<u8>,
    // When a command runs the previous command stops
}

impl Dispatcher {
    fn new() -> Self {
        // Function tables for control keys on LPX.  Keys are the MIDI
        // bytes passed from the pad and the values are the commands,
        // as String.  A command string refers to an executable file
        // in <PWD>/subs/.  Not every control key needs to be here,
        // and if an executable does not exist it is ignored.

        // Each button on the device has two commands associated with
        // it: Firstly when it is activated.  Second when a button is
        // pressed after it has been activated.  "ON" and "OFF".

        // The commands for each button (executable files under
        // <PWD>/subs/) are named: ON-Ctl.N and OFF-Ctl-N where N is
        // MIDI value.
        let mut up_table: HashMap<u8, String> = HashMap::new();
        let mut down_table: HashMap<u8, String> = HashMap::new();

        // When a button pressed store its MIDI value here
        let last: Option<u8> = None;

        up_table.insert(19, "ON-CTL.19".to_string());
        up_table.insert(29, "ON-CTL.29".to_string());
        up_table.insert(39, "ON-CTL.39".to_string());
        up_table.insert(49, "ON-CTL.49".to_string());
        up_table.insert(59, "ON-CTL.59".to_string());
        up_table.insert(69, "ON-CTL.69".to_string());
        up_table.insert(79, "ON-CTL.79".to_string());
        up_table.insert(89, "ON-CTL.89".to_string());

        down_table.insert(19, "OFF-CTL.19".to_string());
        down_table.insert(29, "OFF-CTL.29".to_string());
        down_table.insert(39, "OFF-CTL.39".to_string());
        down_table.insert(49, "OFF-CTL.49".to_string());
        down_table.insert(59, "OFF-CTL.59".to_string());
        down_table.insert(69, "OFF-CTL.69".to_string());
        down_table.insert(79, "OFF-CTL.79".to_string());
        down_table.insert(89, "OFF-CTL.89".to_string());

        Self {
            down_table: down_table,
            up_table: up_table,
            last: last,
        }
    }
    fn run_cmd(cmd: &str) {
        // log(format!("Run down: {}", &cmd).as_str());
        let command = format!(
            "{}/subs/{}",
            env::current_dir()
                .unwrap()
                .as_path()
                .as_os_str()
                .to_str()
                .unwrap(),
            &cmd
        );

        match process::Command::new(command.as_str()).output() {
            Ok(out) => {
                if out.status.success() {
                    let s = String::from_utf8_lossy(&out.stdout);
                    log(format!("Success: {} and stdout was:\n{}", cmd, s).as_str())
                }
            }
            Err(err) => log(format!("Failure: cmd {}  Err: {:?}", cmd, err).as_str()),
        }
    }
    // Make a
    fn set_pad_colour(pad: u8, colour: u8) -> Result<(), Box<dyn Error>> {
        // This is the MIDI message that puts the LPX into programmer's
        // mode.

        let msg: [u8; 10] = [240, 0, 32, 42, 12, 3, 0, pad, colour, 247];
        println!("pad colour: {:?}", msg);
        // The port that we will send the message to
        let mut port: Option<MidiOutputPort> = None;
        let source_port = "Launchpad X:Launchpad X MIDI 1".to_string().into_bytes();

        // Get the port by name
        let midi_out = MidiOutput::new("120 Proof")?;
        for (i, p) in midi_out.ports().iter().enumerate() {
            let port_name = midi_out.port_name(p)?.into_bytes();
            let mut accept: bool = true;
            for i in 0..port_name.len() {
                if i < source_port.len() && source_port[i] != port_name[i] {
                    accept = false;
                    break;
                }
            }
            if accept {
                let p = midi_out
                    .ports()
                    .get(i)
                    .ok_or("Invalid port number")?
                    .clone();
                port = Some(p);
                break;
            }
        }
        let out_port = port.unwrap();
        let mut conn_out = midi_out.connect(&out_port, "120 Proof Connection")?;
        conn_out
            .send(&msg)
            .unwrap_or_else(|_| println!("Error when forwarding message ..."));
        Ok(())
    }

    // A control pad has been pressed
    fn run(&mut self, ctl: u8) {
        // Shut down the last control used
        if let Some(x) = self.last {
            match self.down_table.get(&x) {
                Some(cmd) => {
                    // There is a command to run for shutting down last control
                    Self::run_cmd(cmd.as_str());
                }
                // The last control does not need anything special to
                // shutdown
                None => (),
            }
        }
        self.last = Some(ctl);

        match self.up_table.get(&ctl) {
            Some(cmd) => {
                Self::run_cmd(cmd.as_str());
            }
            None => (),
        };
    }
}

fn main() {
    let dispatcher = Dispatcher::new();
    Dispatcher::set_pad_colour(19, 5).unwrap();
    Dispatcher::set_pad_colour(18, 61).unwrap();
    Dispatcher::set_pad_colour(17, 51).unwrap();
    Dispatcher::set_pad_colour(11, 5).unwrap();
    match run(dispatcher) {
        Ok(_) => (),
        Err(err) => log(format!("Error: {}", err).as_str()),
    }
}

// Process a MIDI message
fn process_message(message: &[u8; 3], dispatcher: &mut Dispatcher) {
    if message[0] == 176 {
        // A ctl message

        let key = message[1];
        let vel = message[2];
        if key >= 19 {
            // There is some noise coming from the LPX with ctl-key 7
            // The rest are control signals that we want
            if vel > 0 {
                // 0 VEL is pad release
                dispatcher.run(key);
            }
        }
    }
}
fn run(dispatcher: Dispatcher) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("120 Proof")?;
    midi_in.ignore(Ignore::None);

    // The port we get messages on
    let mut port: Option<MidiInputPort> = None;
    let source_port = "Launchpad X:Launchpad X MIDI 2".to_string().into_bytes();

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    for (i, p) in in_ports.iter().enumerate() {
        let port_name = midi_in.port_name(p)?.into_bytes();
        let mut accept: bool = true;
        for i in 0..port_name.len() {
            if i < source_port.len() && source_port[i] != port_name[i] {
                accept = false;
                break;
            }
        }
        if accept {
            let p = midi_in.ports().get(i).ok_or("Invalid port number")?.clone();
            port = Some(p);
            break;
        }
    }
    let in_port = port.unwrap();

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |_stamp, message, dispatcher| {
            // println!(
            //     "{}: Msg: {:?} (len = {})",
            //     (stamp as f64) / 1_000_000.0,
            //     &message,
            //     message.len()
            // );
            if message.len() == 3 {
                let array = <[u8; 3]>::try_from(message).unwrap();
                process_message(&array, dispatcher);
            }
        },
        dispatcher,
    )?;

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    log(format!("Closing connection").as_str());
    Ok(())
}
