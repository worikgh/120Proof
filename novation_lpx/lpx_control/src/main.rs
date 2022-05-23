//! Use the MIDI control keys from the LPX to run programes.
use midi_connection::MIDICommunicator;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::stdin;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

/// Dispatcher matches a control key to an executable and executes it
struct Dispatcher {
    // Associate a control value with a command. When a command runs
    // the previous command stops
    up_table: HashMap<u8, String>,
    down_table: HashMap<u8, String>,
    last: Option<u8>,
}
impl std::fmt::Debug for Dispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
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
        eprintln!("Run down: {}", &cmd);
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
                    eprintln!("Success: {} and stdout was:\n{}", cmd, s)
                }
            }
            Err(err) => eprintln!("Failure: cmd {}  Err: {:?}", cmd, err),
        }
    }
    /// A control pad has been pressed
    /// `ctl` is the pad number
    fn run_ctl(&mut self, ctl: u8) {
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

#[derive(Debug)]
struct LpxControl {
    // The source of truth for the state of the controls.
    lpx_enabled: Arc<Mutex<bool>>,

    // Used to put controls to sleep when a MIDI key pressed
    // lpx_midi_sink: MIDICommunicator<()>,
    sleeping: Arc<Mutex<usize>>,
    // Change the colours of the LPX to reflect enabled/disabled
    // state.  Has to be called from the main thread of this class
    // that controls how long the LPX Ctl pads are inactive after
    // notes sent
    lpx_midi: Arc<Mutex<MIDICommunicator<()>>>,
}
impl LpxControl {
    fn new() -> LpxControl {
        LpxControl {
            sleeping: Arc::new(Mutex::new(0)),
            lpx_enabled: Arc::new(Mutex::new(true)),
            lpx_midi: Arc::new(Mutex::new(
                MIDICommunicator::new(
                    "Launchpad X:Launchpad X MIDI 1",
                    "120-Proof-CTL",
                    move |_, _, _| {},
                    (),
                    2,
                )
                .unwrap(),
            )),
        }
    }
    fn sleeping(&self) -> bool {
        let g = self.sleeping.lock().unwrap();
        // eprintln!("Sleeping ? ({})", g);
        *g != 0
    }
    fn sleep(&self, s: usize) {
        let mut g = self.sleeping.lock().unwrap();
        *g += s;
        let lpx_enabled = self.lpx_enabled.clone();
        let lpx_midi = self.lpx_midi.clone();

        let mut lpx_midi_mut = lpx_midi.lock().unwrap();
        let mut lpx_enabled_mut = lpx_enabled.lock().unwrap();
        enable_lpx(false, &mut lpx_midi_mut, &mut lpx_enabled_mut);

        // eprintln!("Put to sleep: sleeping({})", g);
    }

    /// Starts the thread that monitors `sleeping`.  It maintains a
    /// local counter `c`, wakes up periodically and increments `c`.
    /// It then compares the value of `c` to `sleeping`.  If `c >
    /// sleeping` then both `c` and `sleeping` reset to 0 and lpx is
    /// enabled.  Otherwise the lpx is dissabled.
    fn start(&self) {
        let sleeping = self.sleeping.clone();
        let lpx_enabled = self.lpx_enabled.clone();
        let lpx_midi = self.lpx_midi.clone();

        // Initialise
        {
            let mut lpx_midi_mut = lpx_midi.lock().unwrap();
            let mut lpx_enabled_mut = lpx_enabled.lock().unwrap();
            *lpx_enabled_mut = false; // Force an update
            enable_lpx(true, &mut lpx_midi_mut, &mut lpx_enabled_mut);
        }

        thread::spawn(move || {
            let mut c = 0;
            loop {
                sleep(Duration::new(1, 0));
                c += 1;
                let mut sleeping_mut = sleeping.lock().unwrap();
                let mut lpx_midi_mut = lpx_midi.lock().unwrap();
                // eprintln!("In thread: c({}) g({})", &c, sleeping_mut);
                let mut lpx_enabled_mut = lpx_enabled.lock().unwrap();
                if c >= *sleeping_mut {
                    *sleeping_mut = 0;
                    c = 0;
                    enable_lpx(true, &mut lpx_midi_mut, &mut lpx_enabled_mut);
                }
            }
        });
    }
}

#[derive(Debug)]
struct MidiCommTools {
    dispatcher: Dispatcher,
    lpx_control: LpxControl,
}
impl MidiCommTools {
    fn new() -> Self {
        let dispatcher = Dispatcher::new();
        let lpx_control = LpxControl::new();
        lpx_control.start();
        Self {
            lpx_control: lpx_control,
            dispatcher: dispatcher,
        }
    }
}

/// Process a MIDI message
fn process_message(message: &[u8; 3], dispatcher: &mut Dispatcher) {
    if message[0] == 176 {
        // A ctl message
        // eprintln!("process_message message({:?})", message);

        let key = message[1];
        let vel = message[2];
        if key >= 19 {
            // There is some noise coming from the LPX with ctl-key 7
            // The rest are control signals that we want
            if vel > 0 {
                // 0 VEL is pad release
                dispatcher.run_ctl(key);
            }
        }
    }
}

/// Change the colour of the control pads.  Depending on the parameter
/// `enable`.  If `enable` is true the pads are being enabled and are
/// coloured green (87) and if !enabled the pads are being disabled
/// and are coloured red (5)
fn enable_lpx(enable: bool, lpx_midi: &mut MIDICommunicator<()>, lpx_enabled: &mut bool) {
    if *lpx_enabled != enable {
        // eprintln!("enable_lpx: enable({})", enable);
        let pad_colour = if enable { 87 } else { 5 };
        for i in 1..9 {
            let p = i * 10 + 9; // Pad
            let out_message_colour_change: [u8; 11] =
                [240, 0, 32, 41, 2, 12, 3, 0, p, pad_colour, 247];
            match lpx_midi.send(&out_message_colour_change) {
                Ok(()) => (),
                Err(err) => eprintln!("Failed send: {:?}", err),
            };
        }
        *lpx_enabled = enable;
    }
}

/// Main loop.  
fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    eprintln!("Started lpx_control");
    let midi_comm_tools = MidiCommTools::new();
    let _conn_in = MIDICommunicator::new(
        "Launchpad X:Launchpad X MIDI 2",
        "120-Proof-CTL",
        move |_stamp, message, midi_comm_tools| {
            // eprintln!(
            //     "{}: Msg: {:?} (len = {})",
            //     (stamp as f64) / 1_000_000.0,
            //     &message,
            //     message.len()
            // );

            if message.len() == 3 {
                // eprintln!(
                //     "Got a message({:?}) sleeping({}) ",
                //     message,
                //     midi_comm_tools.lpx_control.sleeping()
                // );
                if !midi_comm_tools.lpx_control.sleeping() {
                    if message[0] == 176 {
                        let array = <[u8; 3]>::try_from(message).unwrap();
                        process_message(&array, &mut midi_comm_tools.dispatcher);
                    } else if message[0] == 144 {
                        // A MIDI note
                        //    if lpx_control.sleeping
                        midi_comm_tools.lpx_control.sleep(3);
                    }
                }
            }
        },
        midi_comm_tools,
        1,
    )?;

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    eprintln!("Closing connection");
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err),
    }
}
