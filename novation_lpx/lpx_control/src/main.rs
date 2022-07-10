//! Use the MIDI control keys from the LPX to run programmes.
// use std::io::stdin;
use midi_connection::MIDICommunicator;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::Path;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

// Colours used for the keys to provide feedback
static ENABLEDCOLOUR: u8 = 87; // Ready
static DISABLEDCOLOUR: u8 = 5; // Disabled
static SELECTEDCOLOUR: u8 = 67; // In use

/// Dispatcher matches a control key to an executable and executes it
struct Dispatcher {
    // Associate a control value with a command. When a command runs
    // the previous command stops
    up_table: HashMap<u8, String>,
    down_table: HashMap<u8, String>,

    // The last pad pressed.  At the start this is None
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
        eprintln!("run_cmd({}) Starts", &cmd);
        let mut one_20_proof_home: String = ".".to_string();
        for (key, value) in env::vars() {
            if key == "Home120Proof" {
                one_20_proof_home = value;
                break;
            }
        }
        let home_dir = format!("{}/subs", &one_20_proof_home);
        let home_dir = Path::new(home_dir.as_str());
        env::set_current_dir(&home_dir)
            .expect(format!("Cannot change directory to: {}", home_dir.display()).as_str());
        let command = format!("{}/{}", home_dir.display(), &cmd);
        eprintln!("run_cmd({}) Command: {}", &cmd, &command);
        match process::Command::new(command.as_str()).output() {
            Ok(out) => {
                if out.status.success() {
                    let s = String::from_utf8_lossy(&out.stdout);
                    eprintln!("Success: {} and stdout was:\n{}", cmd, s)
                } else {
                    let s = String::from_utf8_lossy(&out.stderr);
                    eprintln!("Not success: {} and stderr was:{}", cmd, s)
                }
            }
            Err(err) => eprintln!("Failure: cmd {}  Err: {:?}", command, err),
        }
        eprintln!("run_cmd({}) Ends", &cmd);
    }

    /// A control pad has been pressed
    /// `ctl` is the pad number
    fn run_ctl(&mut self, ctl: u8, lpx_midi: Arc<Mutex<MIDICommunicator<()>>>) {
        // Shut down the last control used
        eprintln!("run_ctl({}) Starts", ctl);
        if let Some(x) = self.last {
            eprintln!("There was a last: {}", &x);
            match self.down_table.get(&x) {
                Some(cmd) => {
                    // There is a command to run for shutting down last control

                    // Start flashing the selected pad to illustrate it is turning off
                    let out_message_flash: [u8; 11] =
                        [240, 0, 32, 41, 2, 12, 3, 2, x, SELECTEDCOLOUR, 247];
                    let mut midi_comm = lpx_midi.lock().unwrap();
                    match midi_comm.send(&out_message_flash) {
                        Ok(()) => eprintln!("Sent message: {:?}", &out_message_flash),
                        Err(err) => eprintln!("Failed send: {:?}", err),
                    };

                    Self::run_cmd(cmd.as_str());

                    // Colour the pad enabled
                    let out_message_disable: [u8; 11] =
                        [240, 0, 32, 41, 2, 12, 3, 0, x, ENABLEDCOLOUR, 247];
                    match midi_comm.send(&out_message_disable) {
                        Ok(()) => eprintln!("Sent message: {:?}", &out_message_disable),
                        Err(err) => eprintln!("Failed send: {:?}", err),
                    };
                }
                // The last control does not need anything special to
                // shutdown
                None => (),
            }
        }
        self.last = Some(ctl);

        match self.up_table.get(&ctl) {
            Some(cmd) => {
                eprintln!("run_ctl({}) Run command: {}", ctl, &cmd);
                // Flash pad to show it is being enabled
                let out_message_flash: [u8; 11] =
                    [240, 0, 32, 41, 2, 12, 3, 2, ctl, SELECTEDCOLOUR, 247];
                let mut midi_comm = lpx_midi.lock().unwrap();
                match midi_comm.send(&out_message_flash) {
                    Ok(()) => eprintln!("Sent message: {:?}", &out_message_flash),
                    Err(err) => eprintln!("Failed send: {:?}", err),
                };

                Self::run_cmd(cmd.as_str());

                // Colour pad selected
                let out_message_enable: [u8; 11] =
                    [240, 0, 32, 41, 2, 12, 3, 0, ctl, SELECTEDCOLOUR, 247];
                match midi_comm.send(&out_message_enable) {
                    Ok(()) => eprintln!("Sent message: {:?}", &out_message_enable),
                    Err(err) => eprintln!("Failed send: {:?}", err),
                };
            }
            None => (),
        };
        eprintln!("run_ctl({}) finish", ctl);
    }
}

#[derive(Debug)]
struct LpxControl {
    // The counter the monitoring thread uses.  When put to sleep
    // `sleeping` is set to this plus 3
    counter: Arc<Mutex<usize>>,

    // The source of truth for the state of the controls.  Includes the selected pad if there is one
    lpx_enabled: Arc<Mutex<(bool, Option<u8>)>>,

    // Used to put controls to sleep when a MIDI key pressed
    // lpx_midi_sink: MIDICommunicator<()>,
    sleeping: Arc<Mutex<usize>>,

    // Change the colours of the LPX to reflect enabled/disabled
    // state.  Has to be called from the main thread of this class
    // that controls how long the LPX Ctl pads are inactive after
    // notes sent.  It is also used to change the colour of the
    // selected key
    lpx_midi: Arc<Mutex<MIDICommunicator<()>>>,
}
impl LpxControl {
    fn new() -> LpxControl {
        LpxControl {
            sleeping: Arc::new(Mutex::new(0)),
            lpx_enabled: Arc::new(Mutex::new((true, None))),
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
            counter: Arc::new(Mutex::new(0)),
        }
    }
    fn sleeping(&self) -> bool {
        let g = self.sleeping.lock().unwrap();
        // eprintln!("Sleeping ? ({})", g);
        *g != 0
    }
    fn sleep(&self, s: usize) {
        let mut g = self.sleeping.lock().unwrap();
        let c = self.counter.lock().unwrap();
        *g = *c + s;
        let lpx_enabled = self.lpx_enabled.clone();
        let lpx_midi = self.lpx_midi.clone();

        let mut lpx_midi_mut = lpx_midi.lock().unwrap();
        let mut lpx_enabled_mut = lpx_enabled.lock().unwrap();
        enable_lpx(false, &mut lpx_midi_mut, &mut lpx_enabled_mut);

        // eprintln!("Put to sleep: sleeping({})", (*g - *c));
    }

    /// Starts the thread that monitors `sleeping`.  It maintains a
    /// local counter `c`, wakes up periodically and increments `c`.
    /// It then compares the value of `c` to `sleeping`.  If `c >
    /// sleeping` then both `c` and `sleeping` reset to 0 and lpx is
    /// enabled.  Otherwise the lpx is dissabled.
    fn start(&self) {
        eprintln!("start called");
        let sleeping = self.sleeping.clone();
        let lpx_enabled = self.lpx_enabled.clone();
        let lpx_midi = self.lpx_midi.clone();
        let counter = self.counter.clone();
        // Initialise
        {
            let mut lpx_midi_mut = lpx_midi.lock().unwrap();
            let mut lpx_enabled_mut = lpx_enabled.lock().unwrap();
            lpx_enabled_mut.0 = false; // Force an update
            lpx_enabled_mut.1 = None; // No pad selected
            enable_lpx(true, &mut lpx_midi_mut, &mut lpx_enabled_mut);
        }

        thread::spawn(move || loop {
            sleep(Duration::new(1, 0));
            {
                let mut c = counter.lock().unwrap();
                *c += 1;
                let mut sleeping_mut = sleeping.lock().unwrap();
                let mut lpx_midi_mut = lpx_midi.lock().unwrap();
                // eprintln!("In thread: c({}) g({})", &c, sleeping_mut);
                let mut lpx_enabled_mut = lpx_enabled.lock().unwrap();
                if *c >= *sleeping_mut {
                    *sleeping_mut = 0;
                    *c = 0;

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
fn process_message(
    message: &[u8; 3],
    dispatcher: &mut Dispatcher,
    lpx_midi: Arc<Mutex<MIDICommunicator<()>>>,
) {
    eprintln!("process_message({:?})", message);
    if message[0] == 176 {
        // A ctl message
        eprintln!("process_message message({:?})", message);

        let key = message[1];
        let vel = message[2];
        if key >= 19 {
            // There is some noise coming from the LPX with ctl-key 7
            // The rest are control signals that we want
            if vel > 0 {
                // 0 VEL is pad release
                dispatcher.run_ctl(key, lpx_midi);
            }
        }
    }
}

/// Change the colour of the control pads.  Depending on the parameter
/// `enable`.  If `enable` is true the pads are being enabled and are
/// coloured green (87) and if !enabled the pads are being disabled
/// and are coloured red (5)
fn enable_lpx(
    enable: bool,
    lpx_midi: &mut MIDICommunicator<()>,
    lpx_enabled: &mut (bool, Option<u8>),
) {
    if lpx_enabled.0 != enable {
        // eprintln!("enable_lpx: enable({})", enable);
        let pad_colour = if enable {
            ENABLEDCOLOUR
        } else {
            DISABLEDCOLOUR
        };
        for i in 1..9 {
            let p = i * 10 + 9; // Pad
            let out_message_colour_change: [u8; 11] =
                [240, 0, 32, 41, 2, 12, 3, 0, p, pad_colour, 247];
            match lpx_midi.send(&out_message_colour_change) {
                Ok(()) => eprintln!("Sent message: {:?}", &out_message_colour_change),
                Err(err) => eprintln!("Failed send: {:?}", err),
            };
        }
        if enable {
            eprintln!("Enabling surface. selected pad: {:?}", lpx_enabled.1);
            // Selected pad must be restored
            match lpx_enabled.1 {
                Some(x) => {
                    let out_message: [u8; 11] =
                        [240, 0, 32, 41, 2, 12, 3, 0, x, ENABLEDCOLOUR, 247];
                    match lpx_midi.send(&out_message) {
                        Ok(()) => eprintln!("Sent message: {:?}", &out_message),
                        Err(err) => eprintln!("Failed send: {:?}", err),
                    };
                }
                None => (),
            }
        }
        *lpx_enabled = (enable, lpx_enabled.1);
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
        move |stamp, message, midi_comm_tools| {
            eprintln!(
                "{}: Msg: {:?} (len = {})",
                (stamp as f64) / 1_000_000.0,
                &message,
                message.len()
            );

            if message.len() == 3 {
                // eprintln!(
                //     "Got a message({:?}) sleeping({}) ",
                //     message,
                //     midi_comm_tools.lpx_control.sleeping()
                // );
                if message[0] == 176 {
                    if !midi_comm_tools.lpx_control.sleeping() {
                        let array = <[u8; 3]>::try_from(message).unwrap();
                        eprintln!("Got a message({:?})", message,);
                        process_message(
                            &array,
                            &mut midi_comm_tools.dispatcher,
                            midi_comm_tools.lpx_control.lpx_midi.clone(),
                        );
                    }
                } else if message[0] == 144 {
                    // A MIDI note
                    midi_comm_tools.lpx_control.sleep(3);
                }
            }
        },
        midi_comm_tools,
        1,
    )?;

    input.clear();
    loop {
        // Infinite loop
        sleep(Duration::new(1, 0));
    }
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err),
    }
}
