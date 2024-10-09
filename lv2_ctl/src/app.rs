//! # Run the user interface
//! Addapted from  Ratatui List example
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md
use crate::colours::ALT_ROW_COLOR;
use crate::colours::COMPLETED_TEXT_COLOR;
use crate::colours::HEADER_BG;
use crate::colours::NORMAL_ROW_COLOR;
use crate::colours::PENDING_TEXT_COLOR;
use crate::colours::SELECTED_STYLE_FG;
use crate::colours::SELECTED_TEXT_FG;
use crate::colours::STATIC_TEXT_FG;
use crate::colours::TEXT_COLOR;
use crate::dialogue::Dialogue;
use crate::dialogue::DialogueError;
use crate::dialogue::DialogueValue;
use crate::lv2::Lv2;
use crate::lv2_simulator::Lv2Simulator;
use crate::lv2_simulator::Status;
use crate::lv2_stateful_list::Lv2StatefulList;
use crate::mod_host_controller::ConDisconFlight;
use crate::mod_host_controller::ModHostController;
use crate::port::ContinuousType;
use crate::port::ControlPortProperties;
use crate::port::Port;
use crate::port::PortType;
use crate::port_table::port_table;
use crate::port_table::value_from_scale_control;
use crate::run_executable;
use color_eyre::config::HookBuilder;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};
use std::{error::Error, io, io::stdout};

/// Used by the scroll bar for LV2 Controls (F2).  So far the scroll
/// bars do nothing at all
const ITEM_HEIGHT: usize = 1;

/// Encodes whether a port value is being incremented `up` or
/// decremented `down` when adjusting the port value.
/// This allows the same code do be used for both cases
#[derive(Debug, PartialEq, Eq)]
enum PortAdj {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
enum AppViewState {
    // Enter the name to save a LV2 simulator, and settings, as
    Lv2SaveName,

    // Listing all simulators
    List,

    // Interacting with mod-host
    Command,
}

/// This struct holds the current state of the app.
pub struct App<'a> {
    // Data from mod-host
    buffer: String,

    /// JACK audio Connections as pairs of ports "<from> <to>"
    jack_connections: HashSet<String>,

    /// Interface to `mod-host`.  Stores the definition of all the
    /// available simulators
    mod_host_controller: &'a mut ModHostController,

    /// Maintain the UI view of all the simulators for the first
    /// screen
    lv2_stateful_list: Lv2StatefulList,

    /// Maintain the view for the second screen of loaded simulators
    lv2_loaded_list: Lv2StatefulList,

    /// The current view
    app_view_state: AppViewState,

    /// The control ports of the simulator selected in the current
    /// views, for views which display such information.  Updated when
    /// the simulater selected
    control_ports: Vec<Port>,

    /// For views that display port values this structure holds them.
    /// It is initialised at the same time the `control_ports` member
    /// is, and maintained in `update_port`.  It maps port symbol
    /// -> port value.  When first initialised the value is unknown,
    /// hence being an option.  Some views will have values ready for
    /// the port when this is created and will set the value and issue
    /// `param_set` commands.  Others will not and will issue param_get
    /// commands and update the value when it arrives.
    port_values: HashMap<String, Option<String>>,

    table_state: TableState,
    scroll_bar_state: ScrollbarState,

    /// Store the last status output so do not thrash status reporting
    /// mechanism (eprintln! as I write) with repeated status messages
    status: Option<String>,

    /// Controls the main loop
    run_app: bool,

    /// When input is needed
    dialogue: Option<Dialogue>,
}

impl Drop for App<'_> {
    /// Ensure the terminal is returned whole to the user on exit
    fn drop(&mut self) {
        restore_terminal().expect("Restore terminal");
    }
}

/// Ensure the terminal is returned whole to the user on panic
fn init_error_hooks() -> color_eyre::Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info)
    }));
    Ok(())
}

/// Set up the terminal for being a TUI
fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> color_eyre::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SaveStruct {
    lv2: Lv2,
    port_values: HashMap<String, Option<String>>,
}
impl App<'_> {
    /// Initialise the App
    pub fn new(mod_host_controller: &mut ModHostController) -> App {
        if let Err(err) = init_error_hooks() {
            eprintln!("1 INFO: {err}: Initialising error hooks");
        }

        // Vec<(name, url)>
        let types: Vec<(String, String)> = mod_host_controller
            .simulators
            .iter()
            .map(|s| (s.name.clone(), s.url.clone()))
            .collect();
        let simulators: Vec<Lv2Simulator> = types
            .iter()
            .enumerate()
            .map(|t| Lv2Simulator {
                name: t.1 .0.clone(),
                status: Status::Unloaded,
                url: t.1 .1.clone(),
                mh_id: t.0, // This is used as mod-host to communicate with loaded simulator
                            // value: None,
            })
            .collect();
        App {
            jack_connections: HashSet::new(),
            buffer: "".to_string(),
            app_view_state: AppViewState::List,
            mod_host_controller,
            lv2_loaded_list: Lv2StatefulList::empty(),
            lv2_stateful_list: Lv2StatefulList::new(simulators),

            table_state: TableState::default().with_selected(0),
            scroll_bar_state: ScrollbarState::default(),
            control_ports: vec![],
            port_values: HashMap::new(),

            // last_mh_command: None,
            // mh_command_queue: VecDeque::new(),
            status: None,

            // Default to running
            // Reset `run` to stop App
            run_app: true,

            dialogue: None,
        }
    }

    fn _ports(&self, lv2_url: &str) -> Vec<&Port> {
        let sim = self
            .mod_host_controller
            .simulators
            .iter()
            .find(|s| s.url == lv2_url)
            .expect("Get the Lv2 for Ports in App");
        sim.ports.iter().collect()
    }

    fn status_string(&self) -> String {
        format!(
         "AppViewState({:?}) queued command({})  Last sent_command({}): {:?}",
         self.app_view_state,
         self.mod_host_controller.mh_command_queue.len(),
         // self.mod_host_controller.mh_command_queue.iter().fold("".to_string(), |a, b| format!("{a}{b}, ")),
         self.mod_host_controller.sent_commands.len(),
         self
            .mod_host_controller
            .sent_commands
            .iter()
            .fold("".to_string(), |a, b| format!("{a}{b}, ")),
      )
    }

    /// Queue a command to send to mod-host
    fn send_mh_cmd(&mut self, cmd: &str) {
        self.mod_host_controller.send_mh_cmd(cmd);
    }

    /// Called from the event loop to send a message to mod-host
    fn pump_mh_queue(&mut self) {
        self.mod_host_controller.pump_mh_queue();
    }

    /// In command mode (f2) when entering this state clean out old
    /// commands that make no sense
    fn filter_command_lists(&mut self, mh_id: usize) {
        let mut new_commands: HashSet<String> = HashSet::new();
        let len = self.mod_host_controller.sent_commands.len();
        let mut itr = self.mod_host_controller.sent_commands.iter();
        for _i in 0..len {
            let cmd = itr.nth(0).unwrap();
            let res = if cmd.starts_with("param_get ")
                || cmd.starts_with("param_set ")
            {
                let p = cmd.as_str()[9..].trim();
                let sp = p.find(' ').unwrap_or(p.len());
                p[0..sp]
                    .parse::<usize>()
                    .expect("param_get should be followed by a usize")
                    == mh_id
            } else if cmd.starts_with("connect effect_") {
                cmd.starts_with(format!("connect effect_{mh_id}").as_str())
            } else {
                true
            };
            if res {
                new_commands.insert(cmd.to_owned());
            }
        }
        self.mod_host_controller.sent_commands = new_commands;
    }

    /// Changes the status of the selected list item.  
    #[allow(clippy::iter_kv_map)]
    fn change_status(&mut self) {
        if self.get_stateful_list().items.is_empty() {
            // Nothing to do
            return;
        }
        match self.app_view_state {
            AppViewState::Lv2SaveName => (),
            AppViewState::List => {
                if let Some(i) = self.get_stateful_list_mut().state.selected() {
                    // There is a selected item at index `i`

                    self.get_stateful_list_mut().items[i].status =
                        match self.get_stateful_list_mut().items[i].status {
                            Status::Unloaded => {
                                // Set status of Lv2 to Loaded.  Put the
                                // simulator into the list of loaded
                                // simulators

                                let lv2: Lv2Simulator =
                                    self.lv2_stateful_list.items[i].clone();

                                let cmd =
                                    format!("add {} {}\n", lv2.url, lv2.mh_id);
                                self.send_mh_cmd(cmd.as_str());

                                // TODO:  Move this into the handler for received messages
                                self.lv2_loaded_list.items.push(lv2);
                                self.lv2_loaded_list
                                    .items
                                    .sort_by(|a, b| a.mh_id.cmp(&b.mh_id));

                                Status::Pending
                            }

                            Status::Loaded => {
                                // Set status of Lv2 to Unloaded.
                                // Remove from the list of loaded
                                // simulators
                                if self.lv2_loaded_list.items.iter().any(|x| {
                                    x.url == self.lv2_stateful_list.items[i].url
                                }) {
                                    let lv2: Lv2Simulator =
                                        self.lv2_stateful_list.items[i].clone();
                                    let cmd = format!("remove {}\n", lv2.mh_id);

                                    self.send_mh_cmd(cmd.as_str());

                                    Status::Unloaded
                                } else {
                                    panic!(
                                    "A loaded LV2 was not om loaded_list.  {i}"
                                );
                                }
                            }

                            Status::Pending => Status::Pending,
                        }
                }
            }
            AppViewState::Command => {
                // Showing and editing the Port values for a LV2.  Change the LV2 simulator
                // TODO: Optimisation - check the same LV2 is not selected twice
                if let Some(idx) = self.get_stateful_list_mut().state.selected()
                {
                    // Connect the selected effect to system in/out
                    let mh_id = self.get_stateful_list().items[idx].mh_id;

                    // Filter out commmands that are waiting to be sent
                    // that are not applicable in the new state
                    self.filter_command_lists(mh_id);

                    let url = self.get_stateful_list().items[idx].url.clone();
                    eprintln!("INFO change_status AppViewState::Command idx: {idx} mh_id: {mh_id} url {url}");
                    self.control_ports = self
                        .mod_host_controller
                        .simulators
                        .iter()
                        .find(|s| s.url == url)
                        .expect("Find LV2 getting ports")
                        .ports
                        .to_vec()
                        .iter()
                        .filter(|p| {
                            p.types
                                .iter()
                                .any(|t| matches!(t, PortType::Control(_)))
                        })
                        .cloned()
                        // .map(|t| t.clone())
                        .collect();
                    self.port_values = self
                        .control_ports
                        .iter()
                        .map(|p| (p.symbol.clone(), None))
                        .collect();

                    // Create commands to get the values
                    let update_port_vales_cmds: Vec<String> = self
                        .port_values
                        .iter()
                        .map(|(s, _)| format!("param_get {mh_id} {s}"))
                        .collect();
                    for cmd in update_port_vales_cmds {
                        self.send_mh_cmd(cmd.as_str());
                    }

                    //  Disconnect any existing connections.  This
                    //  connects one, and only one, LV2
                    let disconnect_cmds = self
                        .jack_connections
                        .iter()
                        .map(|s| format!("disconnect {s}"))
                        .collect::<Vec<String>>();

                    let _control_commands: Vec<String>;
                    let input_commands: Vec<String>;
                    let output_commands: Vec<String>;
                    {
                        let lv_url =
                            self.get_stateful_list().items[idx].url.clone();
                        let lv2 = match self
                            .mod_host_controller
                            .get_lv2_by_url(lv_url.as_str())
                        {
                            Some(l) => l,
                            None => panic!("Getting Lv2 by url"),
                        };

                        let output_ports = lv2
                            .ports
                            .iter()
                            .filter(|p| {
                                p.types.iter().any(|t| t == &PortType::Output)
                                    && p.types
                                        .iter()
                                        .any(|t| t == &PortType::Audio)
                            })
                            .collect::<Vec<&Port>>();
                        let input_ports = lv2
                            .ports
                            .iter()
                            .filter(|p| {
                                p.types.iter().any(|t| t == &PortType::Input)
                                    && p.types
                                        .iter()
                                        .any(|t| t == &PortType::Audio)
                            })
                            .collect::<Vec<&Port>>();
                        eprintln!("DBG output ports: {output_ports:?}");
                        eprintln!("DBG input ports: {input_ports:?}");
                        let mut i = 1;
                        input_commands = input_ports
                            .iter()
                            .map(|p| {
                                let lhs = format!("system:capture_{i}");
                                let rhs = format!(
                                    "effect_{mh_id}:{}",
                                    p.symbol.as_str()
                                );
                                i += 1;
                                format!("connect {lhs} {rhs}")
                            })
                            .collect();
                        let mut i = 1;
                        output_commands = output_ports
                            .iter()
                            .map(|p| {
                                let lhs = format!(
                                    "effect_{mh_id}:{}",
                                    p.symbol.as_str()
                                );
                                let rhs = format!("system:playback_{i}");
                                i += 1;
                                format!("connect {lhs} {rhs}")
                            })
                            .collect();
                        eprintln!("DBG output commands: {output_commands:?}");
                        eprintln!("DBG input commands: {input_commands:?}");
                    }
                    for cmd in disconnect_cmds.iter() {
                        self.send_mh_cmd(cmd.as_str());
                    }
                    for cmd in input_commands.iter() {
                        self.send_mh_cmd(cmd.as_str());
                    }
                    for cmd in output_commands.iter() {
                        self.send_mh_cmd(cmd.as_str());
                    }
                }
            }
        }
    }

    fn go_top(&mut self) {
        self.get_stateful_list_mut().state.select(Some(0))
    }

    fn go_bottom(&mut self) {
        let len = self.get_stateful_list().items.len();
        self.get_stateful_list_mut().state.select(Some(len - 1))
    }

    /// Run the app
    pub fn run(
        mod_host_controller: &mut ModHostController,
    ) -> Result<(), Box<dyn Error>> {
        let terminal = init_terminal()?;
        let mut app = App::new(mod_host_controller);

        app.run_app(terminal).expect("Calling _run");

        restore_terminal()?;

        Ok(())
    }

    /// Set a status line
    fn set_status(&self, status: &str) {
        // No actual status yet
        eprintln!("INFO Status: {status}");
    }

    /// The first integer in the response is <=0 except when a
    /// response to a `param_add` when it is the instance number of
    /// the added simulator,  
    fn validate_resp(&self, resp_code: isize) -> bool {
        if let Ordering::Greater = 0.cmp(&resp_code) {
            return false;
        }
        true
    }

    fn jack_disconnect(&mut self, cmd: &str) -> bool {
        self.jack_connections.remove(cmd)
    }

    /// Handle a response from mod-host that starts with "resp ".  It
    /// is a response to a command, so what happens here is dependant
    /// on that command.  TRhe commands are pushed on the queue
    /// `self.mod_host_controller.last_mh_command` resp status [value]
    fn process_response(&mut self, response: &str) {
        // Can only get a "resp " from mod-host after a command has been sent
        eprintln!("DBG Process response: {response}");
        let resp_code = Self::get_resp_code(response);
        if !self.validate_resp(resp_code) {
            // No action to take if response not valid, except report the error
            let failed_cmd: String = self
                .mod_host_controller
                .resp_command
                .as_ref()
                .unwrap_or(&"<No resp command>".to_string())
                .to_string();
            let error_str = ModHostController::translate_error_code(resp_code);
            eprintln!(
                "DBG CMD FAILED RESP {failed_cmd} -> {error_str}:({resp_code})"
            );
            // Take remedial action if possible
            match resp_code {
            -206 | -205 => {
               // ERR_JACK_PORT_DISCONNECTION
               // A disconnect or connect failed
               assert!(
                  &failed_cmd[0.."disconnect".len()] == "disconnect"
                     || &failed_cmd[0.."connect".len()] == "connect"
               );
               // Resend the command.  Hope not to get into an
               // infinite loop of doom.....
					 eprintln!("ERR Despite error, do not Resend ({error_str}:{resp_code}): {failed_cmd}");
                // self.mod_host_controller.send_mh_cmd(&failed_cmd);
            }
            _ => eprintln!(
               "ERR: Error from mod-host: {error_str}({resp_code}) {failed_cmd}",                
            ),
         };

            return;
        }

        // The command this is in response to
        if self.mod_host_controller.resp_command.is_none() {
            eprintln!("ERR No command for response: {response}");
            return;
        }
        let command = self
            .mod_host_controller
            .resp_command
            .as_ref()
            .unwrap()
            .to_string();
        eprintln!("DBG CMD RESP {command} -> {resp_code}: {response}");
        let command = String::from_utf8(run_executable::resp_0(
            command.as_bytes().to_vec(),
        ))
        .expect("Trim command");
        if command.starts_with("connect") || command.starts_with("disconnect") {
            let (sp, _) = command
                .char_indices()
                .find(|x| x.1 == ' ')
                .expect("No space in (dis)connect command");
            let connection = &command[sp + 1..];
            match (
                self.mod_host_controller.connections.get(connection),
                &command[0..sp],
            ) {
                (Some(ConDisconFlight::Connected), "connect") => {}
                (Some(ConDisconFlight::Disconnected), "connect") => {}
                (Some(ConDisconFlight::InFlight), "connect") => {
                    self.mod_host_controller.connections.insert(
                        connection.to_string(),
                        ConDisconFlight::Connected,
                    );
                }
                (Some(ConDisconFlight::InFlight), "disconnect") => {
                    self.mod_host_controller.connections.insert(
                        connection.to_string(),
                        ConDisconFlight::Disconnected,
                    );
                }
                (Some(ConDisconFlight::Connected), "disconnect") => {}
                (Some(ConDisconFlight::Disconnected), "disconnect") => {}
                (None, b) => eprintln!("No connection state. {b:?}"),
                (a, b) => panic!("Reality discontinuity: {a:?} {b:?}"),
            };
        }

        self.mod_host_controller.resp_command = None;

        // Get the first word as a slice
        let sp: usize = command
            .chars()
            .position(|x| x.is_whitespace())
            .expect("No space in last_mh_command");
        // First word is command
        let cmd = &command[0..sp];

        match cmd {
            "add" => {
                // Adding an LV2.  Get the instance number from the
                // command, the instance number that the command was
                // addressed to
                let sp = command.rfind(' ').unwrap_or_else(|| {
                    panic!("Malformed command: '{command}'")
                });

                let instance_number = command[sp..].trim().parse::<usize>().unwrap_or_else(|_| {
                    panic!(
                        "No instance number at end of add command: '{command}'  sp: {sp} => '{}'",
			&command[sp..]
                    )
                });

                // Get a reference to the item the command is for.
                // Its state will be modified: to `loaded` if all is
                // well, to `unloaded` if mod-host returned an error
                let item: &mut Lv2Simulator = self
                    .lv2_stateful_list
                    .items
                    .iter_mut()
                    .find(|x| x.mh_id == instance_number)
                    .expect("Cannot find LV2 instance: {instance_number}");

                // Get the instance number from the response.  If this
                // is > 0 it is the `instace_number`, else it is an
                // error code
                let n = resp_code;
                if n >= 0 {
                    // `n` is the instance_number of the simulator
                    assert!(
                        n as usize == instance_number,
                        "n:{n} == instance_number:{instance_number}"
                    );

                    item.status = Status::Loaded;
                } else {
                    // Error code
                    let errno = n;
                    eprintln!(
                        "ERR: {errno}.  Command: {:?}: {}",
                        command,
                        ModHostController::translate_error_code(n)
                    );
                    item.status = Status::Unloaded;
                }
            }
            "param_get" => {
                // Command e.g: param_get 50 threshold
                // Response e.g: resp 0 0.1250
                // Got the current value of a Port.
                // Get the symbol for the port from the command
                let q = Self::get_instance_symbol_res(command.as_str());
                let instance_number = q.0;
                let symbol = q.1;
                let value = Self::get_resp_value(response);
                self.update_port(instance_number, symbol, value);
            }
            "param_set" => {
                // E.g: "param_set 1 Gain 0"
                // REsponse e.g: resp 0
                let q = Self::get_instance_symbol_res(command.as_str());
                let instance_number = q.0;
                let symbol = q.1;
                // Set the value in the LV2, update our records
                // Get the value from the command
                let sp = command.len()
                    - command
                        .chars()
                        .rev()
                        .position(|c| c.is_whitespace())
                        .unwrap_or(0);
                let value = command.as_str()[sp..].trim();
                self.update_port(instance_number, symbol, value);
            }
            "remove" => {
                // Removing an LV2.  Get the instance number from the command
                let sp = command.rfind(' ').unwrap_or_else(|| {
                    panic!("Malformed command: '{command}'")
                });
                let instance_number = command[sp..].trim().parse::<usize>().unwrap_or_else(|_| {
                    panic!(
                        "No instance number at end of add command: '{command}'  sp: {sp} => '{}'",
			&command[sp..]
                    )
                });

                // Get response.  If 0, all is good.  Otherwise there
                // is an error.  Leave item pending
                match resp_code.cmp(&0) {
                    Ordering::Equal => {
                        self.lv2_stateful_list
                            .items
                            .iter_mut()
                            .find(|x| x.mh_id == instance_number)
                            .expect(
                                "Cannot find LV2 instance: {instance_number}",
                            )
                            .status = Status::Unloaded
                    }
                    Ordering::Greater => {
                        eprintln!(
                     "ERR: Bad response for {command}.  n > 0: {response}"
                  )
                    }
                    Ordering::Less => eprintln!(
                        "M-H Err: {resp_code} => {}",
                        ModHostController::translate_error_code(resp_code)
                    ),
                };
            }
            "connect" => {
                // A connection was established
                // TODO:  Record connections in model data
                let jacks = &command.as_str()[sp + 1..];
                self.jack_connections.insert(jacks.to_string());
            }
            "disconnect" => {
                let jacks = &command.as_str()[sp + 1..];
                if !self.jack_disconnect(jacks) {
                    eprintln!("ERR Failed to remove {jacks} from connections");
                }
            }
            _ => panic!("Unknown command: {command}"),
        };

        self.mod_host_controller
            .sent_commands
            .remove(command.as_str());
    }

    /// Process data coming from mod-host.  Line orientated and asynchronous
    fn process_buffer(&mut self) {
        // If there is no '\n' in buffer, do not process it, leave it
        // till next time.  But process all lines that are available
        eprintln!("DBG Process Buffer: '{}'", self.buffer);
        while let Some(resp_line) =
            self.buffer.as_str().find(['\n', '\r', '\u{1b}'])
        {
            // There is a line available

            let resp = self.buffer.as_str()[0..resp_line].trim().to_string();

            if !resp.is_empty() {
                // Skip blank lines.
                // The CLI input prompt needs to be filtered out
                let resp =
                    if resp.len() > 8 && &resp.as_str()[0..9] == "mod-host>" {
                        resp[9..].trim()
                    } else {
                        resp.as_str().trim()
                    };

                if !resp.is_empty() {
                    if resp.len() > 5 && &resp[0..5] == "resp " {
                        self.process_response(resp);
                    } else if resp == "using block size: 1024"
                        || resp == "chump"
                        || resp == "bigchump"
                        || resp == "vibrochump"
                        || resp.find(' ').is_none()
                    {
                    } else {
                        self.mod_host_controller.resp_command =
                            Some(resp.to_string());
                    }
                }
            };
            self.buffer = if resp_line < self.buffer.len() {
                self.buffer.as_str()[(resp_line + 1)..].to_string()
            } else {
                "".to_string()
            };
        }
    }

    /// Set a value displayed for the port named by `symbol` to the LV2
    /// with `instance_number`.  Port values are a matter for mod-host
    /// to maintain.  Here they are just displayed.  So check if
    /// `instance_number` matches the currently loaded simulater, and
    /// if so the port's value is stored in `self.control_port_vales`.
    fn update_port(
        &mut self,
        instance_number: usize,
        symbol: &str,
        value: &str,
    ) {
        // Currently loaded simulator

        if let Some(idx) = self.get_stateful_list_mut().state.selected() {
            let mh_id = self.get_stateful_list().items[idx].mh_id;
            if mh_id != instance_number {
                // Simulator was unloaded while command was in flight
                return;
            }
            if self
                .port_values
                .insert(symbol.to_string(), Some(value.to_string()))
                .is_none()
            {}
        }
    }

    /// Get the new value from the response to a `param_get`
    fn get_resp_value(resp: &str) -> &str {
        // resp 0 0.5000
        assert!(resp.len() > 5, "get_resp_value({resp})");
        let r = &resp[6..];
        r.trim()
    }

    fn get_resp_code(resp: &str) -> isize {
        let r = &resp[5..];
        let sp: usize = r.chars().position(|x| x.is_whitespace()).unwrap_or(
            // No whitespace, till end of string
            r.len(),
        );
        let res = r[..sp].trim();
        match res.parse::<isize>() {
            Ok(s) => s,
            Err(err) => panic!("{err}: Not an isize: {res}"),
        }
    }

    /// When responding to a param_get or param_set extract the
    /// instance number, the symbol, and the value (if param_get) from the
    /// simulator from the last command
    fn get_instance_symbol_res(last_mh_command: &str) -> (usize, &str) {
        // Got the current value of a Port.
        // Get the symbol for the port from the command
        // E.g: param_set 2 Volume 0.16717 -> resp 0
        // E.g: param_get 2 Volume -> resp 0 0.3078

        let instance_symbol = last_mh_command["param_get".len()..].trim();
        let sp = instance_symbol
            .find(char::is_whitespace)
            .expect("get_instance_symbol_res: No whitespace in command");
        // Get the instance number from the command
        let instance = instance_symbol[..sp].trim();

        let instance_number = instance.parse::<usize>().unwrap_or_else(|_| {
            panic!("Bad command instance number: {last_mh_command}")
        });

        let symbol = instance_symbol[sp..].trim();

        // param_set has extra data after the symbol, param_get does not
        let sp = symbol.find(char::is_whitespace).unwrap_or(symbol.len());
        let symbol = symbol[..sp].trim();
        (instance_number, symbol)
    }

    pub fn load_lv2(&mut self, f_n: &str) -> bool {
        let mut _file = match File::open(f_n) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("Saving LV2 to {f_n}.  Failed to open file: {err:?}");
                return false;
            }
        };

        let json: String = std::fs::read_to_string(f_n)
            .expect("Reading LV2.  Failed to open and read file")
            .lines()
            .map(String::from)
            .collect::<Vec<String>>()
            .join("");
        let sj: SaveStruct = match serde_json::from_str(json.as_str()) {
            Ok(st) => st,
            Err(err) => {
                eprintln!("{err}: Failed to convert file: {f_n}");
                return false;
            }
        };
        let lv2: Lv2 = sj.lv2;

        let port_values = sj.port_values;
        // let mut new_items:Vec<Lv2Simulator> =
        match self.lv2_stateful_list.mk_lv2_simulator(&lv2) {
            Ok(s) => {
                let cmd = format!("add {} {}\n", s.url.as_str(), s.mh_id,);

                self.send_mh_cmd(cmd.as_str());
                for (k, v) in port_values.iter() {
                    let cmd = format!(
                        "param_set {} {k} {}\n",
                        s.mh_id,
                        v.as_ref().unwrap()
                    );

                    self.send_mh_cmd(cmd.as_str());
                }
                self.lv2_stateful_list.items.push(s.clone());
                self.lv2_stateful_list =
                    Lv2StatefulList::new(self.lv2_stateful_list.items.clone());
                self.lv2_loaded_list.items.push(s);
            }
            Err(_err) => panic!("Cannot make Lv2Simulator"),
        };
        self.mod_host_controller.simulators.push(lv2);

        false
    }

    pub fn save_lv2(&self, f_n: &str) -> bool {
        if let Some(url) = self.get_stateful_list().get_selected_url() {
            let lv2: Lv2 =
                match self.mod_host_controller.get_lv2_by_url(url.as_str()) {
                    Some(u) => u.clone(),
                    None => panic!("save LV2, cannot load url: {url}"),
                };
            let port_values = self.port_values.clone();
            let save_struct = SaveStruct { lv2, port_values };
            let save_string = serde_json::to_string_pretty(&save_struct)
                .expect("Serialising LV2 data to save");
            let mut file = match File::create(f_n) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!(
                  "ERR Saving LV2 to {f_n}.  Failed to open file: {err:?}"
               );
                    return false;
                }
            };
            match file.write_all(save_string.as_bytes()) {
                Err(err) => {
                    eprintln!(
                  "ERR Saving LV2 to {f_n}.  Failed to write file: {err:?}"
               );
                    false
                }
                Ok(_) => true,
            }
        } else {
            eprintln!("ERR Saving LV2 to {f_n}.  Nothing selected");
            false
        }
    }

    /// There is a port in the UI focus
    /// being adjusted up, or down
    fn handle_port_adj(&mut self, adj: PortAdj, k: &KeyEvent) {
        // Get the port
        // let port = self.control_ports.iter().nth(self.table_state.selected().unwrap()).unwrap();
        let mh_id: usize;
        let port: &Port;
        if let Some(idx) = self.get_stateful_list_mut().state.selected() {
            // Connect the selected effect to system in/out

            mh_id = self.get_stateful_list().items[idx].mh_id;
            if let Some(i) = self.table_state.selected() {
                port = self.control_ports.get(i).unwrap();
            } else {
                return;
            }
        } else {
            eprintln!("ERR Attempting to adjust a port when there is no simulator selected");
            return;
        }
        // Symbol
        let port_symbol = port.symbol.clone();
        let value: String = match self.port_values.get(port_symbol.as_str()) {
            Some(v) => match v {
                Some(s) => s.clone(),
                _ => {
                    eprintln!("Must have a value to adjust");
                    return;
                }
            },
            None => panic!("Unknown port symbol"),
        };

        // Get the ControlPort interface
        for pt in port.types.iter() {
            if let PortType::Control(ptc) = pt {
                // Do adjustment
                let cpp: &ControlPortProperties = ptc;
                match cpp {
                    ControlPortProperties::Continuous(cppc) => {
                        // Adjust between max and min
                        let range = cppc.max - cppc.min;
                        let n: usize = 128; // 128 graduations of a MIDI control
                        let _step = range / n as f64;

                        let v = value
                            .parse::<f64>()
                            .expect("Value should be a valid number");
                        let n: f64 = if cppc.logarithmic { v.ln() } else { v };
                        let n = if k.modifiers.contains(KeyModifiers::SHIFT) {
                            // Shift key pressed.  Move half the distance
                            // between `v` and `max` for adj == PortAdj::Up
                            // and `min` if PortAdj::Down
                            let max = if cppc.logarithmic {
                                cppc.max.ln()
                            } else {
                                cppc.max
                            };
                            let min = if cppc.logarithmic {
                                cppc.min.ln()
                            } else {
                                cppc.min
                            };
                            match adj {
                                PortAdj::Down => min + (v - min) / 2.0,
                                PortAdj::Up => max - (max - v) / 2.0,
                            }
                        } else {
                            n + _step
                                * match adj {
                                    PortAdj::Down => -1_f64,
                                    PortAdj::Up => 1_f64,
                                }
                        };

                        let n: f64 = if cppc.logarithmic { n.exp() } else { n };
                        let n = if n > cppc.max { cppc.max } else { n };
                        let n = if n < cppc.min { cppc.min } else { n };

                        // `n` is the updated value
                        let new_value = match cppc.kind {
                            ContinuousType::Decimal => format!("{:.2}", n),
                            ContinuousType::Integer => format!("{:.0}", n),
                            ContinuousType::Double => format!("{n:.4}"),
                        };
                        let new_value = new_value.trim();
                        let cmd = format!(
                            "param_set {mh_id} {port_symbol} {new_value}"
                        );
                        self.mod_host_controller.send_mh_cmd(cmd.as_str());
                    }
                    ControlPortProperties::Scale(cpps) => {
                        if let Some(value_idx) =
                            cpps.labels_values.iter().position(|lv| {
                                lv.1 == value_from_scale_control(value.as_str())
                            })
                        {
                            // if let Some(value_idx) = cpps.value {
                            let new_value = if adj == PortAdj::Down {
                                if value_idx == 0 {
                                    // Nothing to do,  Cannot go below zero
                                    return;
                                }
                                cpps.labels_values[value_idx - 1].1.clone()
                            } else {
                                if value_idx + 1 == cpps.labels_values.len() {
                                    // Nothing to do.  Cannot go any higher
                                    return;
                                }
                                cpps.labels_values[value_idx + 1].1.clone()
                            };
                            let cmd = format!(
                                "param_set {mh_id} {port_symbol} {new_value}"
                            );
                            self.mod_host_controller.send_mh_cmd(cmd.as_str());
                        } else {
                            eprintln!("ERR: handle_port_adj No value when adjusting {port_symbol}");
                        }
                    }
                }
            }
        }
    }

    /// Entering a file name to use to save a LV2 as JSON.  
    fn key_enter_filename(&mut self, key: &KeyEvent) {
        match self.dialogue.as_mut().map(|d| d.handle_key(key)) {
            Some(Ok(DialogueValue::Continue)) => (),
            Some(Ok(DialogueValue::Final(save_name))) => {
                // Test to see if file exists.  If so, load it, if not create it
                self.app_view_state = AppViewState::Command;
                let save_name = format!("data/{save_name}.json");
                if Path::new(save_name.as_str()).exists() {
                    self.load_lv2(save_name.as_str());
                    self.dialogue = None;
                } else if self.save_lv2(save_name.as_str()) {
                    self.dialogue = None;
                } else {
                    self.app_view_state = AppViewState::Lv2SaveName;
                }
            }
            Some(Err(DialogueError::Close)) => {
                self.dialogue = None;
                self.app_view_state = AppViewState::Command;
            }
            None => {
                panic!("{:?} No dialogue?", key.code);
            }
        }
    }

    fn handle_key_display(&mut self, key: &KeyEvent) {
        use KeyCode::*;

        match key.code {
            Left => {
                // In Port Control window decrease value of
                // port by one unit
                if self.app_view_state == AppViewState::Command {
                    self.handle_port_adj(PortAdj::Down, key);
                }
            }
            Right => {
                // In Port Control window increase value of
                // port by one unit
                if self.app_view_state == AppViewState::Command {
                    self.handle_port_adj(PortAdj::Up, key);
                }
            }
            Down => self.get_stateful_list_mut().next(),
            Up => self.get_stateful_list_mut().previous(),
            Char('q') | Esc => {
                self.send_mh_cmd("quit");
                // Move this to handler of data from mod-host?
                //return Ok(());
                self.run_app = false;
            }
            Char('u') => self.get_stateful_list_mut().unselect(),
            Char('g') => self.go_top(),
            Char('G') => self.go_bottom(),
            Char('n') => {
                // In LV2 Control view (F2) move down
                // on Port in Port display
                if self.app_view_state == AppViewState::Command {
                    // In LV2 Control view (F2) move down a port
                    let i = match self.table_state.selected() {
                        Some(i) => {
                            if i >= self.control_ports.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    self.table_state.select(Some(i));
                    self.scroll_bar_state =
                        self.scroll_bar_state.position(i * ITEM_HEIGHT);
                }
            }
            Char('p') => {
                // In LV2 Control view (F2) move up
                // on Port in Port display
                if self.app_view_state == AppViewState::Command {
                    // In LV2 Control view (F2) move down
                    let i = match self.table_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                self.control_ports.len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    self.table_state.select(Some(i));
                    self.scroll_bar_state =
                        self.scroll_bar_state.position(i * ITEM_HEIGHT);
                }
            }
            Char('s') => {
                if self.app_view_state == AppViewState::Command {
                    self.app_view_state = AppViewState::Lv2SaveName;
                    self.dialogue =
                        Some(Dialogue::new("Enter name for simulator"));
                }
            }
            Enter => self.change_status(),
            // Function keys for setting modes
            F(1) => self.app_view_state = AppViewState::List,
            F(2) => self.app_view_state = AppViewState::Command,
            _ => {
                eprintln!(
               "INFO Unrecognised key code: {:?} Modifier: {:?} Control: {}",
               key.code,
               key.modifiers,
               key.modifiers & crossterm::event::KeyModifiers::CONTROL
                  == crossterm::event::KeyModifiers::CONTROL
            );
            }
        }
    }

    /// A key event is detected. User input
    fn handle_key(&mut self, key: &KeyEvent) {
        match self.app_view_state {
            AppViewState::Command | AppViewState::List => {
                self.handle_key_display(key);
            }
            AppViewState::Lv2SaveName => {
                self.key_enter_filename(key);
            }
        }
    }

    /// The main body of the App
    fn run_app(
        &mut self,
        mut terminal: Terminal<impl Backend>,
    ) -> io::Result<()> {
        // init_error_hooks().expect("App::run error hooks");

        // Control the event loop.  `frame_time` is the Duration of a loop.
        let target_fps = 100; // 400 is about the limit on Raspberry Pi 5
        let frame_time = Duration::from_secs(1) / target_fps as u32;

        // Record the instant the loop started for debugging
        let _instant_loop_started = Instant::now();
        let mut _tick_counter = 0; // Reset every debug report
        loop {
            // Provide the world with a message that the event loop is
            // spinning
            _tick_counter += 1;
            if _tick_counter % (60 * target_fps) == 0 {
                let d = _instant_loop_started.elapsed();
                eprintln!("Event loop tick {_tick_counter}  {d:?}");
                //_tick_counter = 0;
            }

            let start_time = Instant::now();
            if !self.run_app {
                break;
            }

            // let status = "".to_string();
            let status = self.status_string();
            if let Some(st) = &self.status {
                if st != &status {
                    self.set_status(status.as_str());
                    self.status = Some(status);
                }
            } else {
                self.set_status(status.as_str());
                self.status = Some(status);
            }

            self.draw(&mut terminal)?;

            if event::poll(Duration::from_secs(0))
                .expect("Polling for event from Ratatui")
            {
                let ev = event::read();
                match ev {
                    Ok(Event::Key(key)) => {
                        self.handle_key(&key);
                    }
                    Ok(Event::Resize(_, _)) => (),
                    Err(err) => panic!("{err}: Reading event"),
                    x => panic!("Error reading event: {x:?}"),
                };
            }

            // Send data to mod-host if it is enqueued
            self.pump_mh_queue();

            // Is there any data from mod-host
            if let Ok(Some(data)) = self.mod_host_controller.try_get_resp() {
                // // Clean up the data.
                // let data_b = data.as_bytes();
                self.buffer += data.as_str();
                self.process_buffer();
            };

            // Maintain timing of loop
            let elapsed_time = Instant::now() - start_time;
            if elapsed_time < frame_time {
                thread::sleep(frame_time - elapsed_time);
            } else {
                eprintln!("Timing error: {elapsed_time:?}/{frame_time:?}");
            }
        }
        Ok(())
    }

    fn draw(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
    ) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.area()))?;
        Ok(())
    }

    /// Get the StateFulList that is currently in view
    fn get_stateful_list(&self) -> &Lv2StatefulList {
        match self.app_view_state {
            AppViewState::Lv2SaveName => panic!(
                "Do not call get_stateful_list in this state {:?}",
                AppViewState::Lv2SaveName
            ),
            AppViewState::List => &self.lv2_stateful_list,
            AppViewState::Command => &self.lv2_loaded_list,
        }
    }

    /// Get a mutable reference to the StateFulList that is currently
    /// in view
    fn get_stateful_list_mut(&mut self) -> &mut Lv2StatefulList {
        match self.app_view_state {
            AppViewState::Lv2SaveName => panic!(
                "Do not call get_stateful_list_mut in this state {:?}",
                AppViewState::Lv2SaveName
            ),
            AppViewState::List => &mut self.lv2_stateful_list,
            AppViewState::Command => &mut self.lv2_loaded_list,
        }
    }

    fn render_save_name(&mut self, area: Rect, buf: &mut Buffer) {
        if let Some(d) = self.dialogue.as_mut() {
            d.render(area, buf)
        }
    }

    /// F1 The main screen with all known simulators displayed.
    /// Simulators can be loaded here.  For now simulators can only be
    /// loaded once.
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        // Header, body, and footer
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);
        // Create a space for header,  list and the footer.

        // Create two chunks with equal vertical screen space. One for the list and the other for
        // the info block.
        let v = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]);
        let [upper_item_list_area, lower_item_list_area] = v.areas(rest_area);

        self.render_title(header_area, buf);
        self.render_lv2_list(upper_item_list_area, buf);
        self.render_details(lower_item_list_area, buf);
        self.render_footer(footer_area, buf);
    }

    /// F2 The LV2 simulators that were selected in the main (F1)
    /// screen are all listed here in the top part of the screen, in a
    /// list.  The simulator selected there has its control ports
    /// listed in the bottom part of the screen.
    fn render_selected_lv2(&mut self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        // Create two chunks: Top chunk is (25%) is the list of
        // selected devices (that are loaded into mod-host with
        // `add...`) and the bottom 75% is for configuring the
        // simulator
        let vertical = Layout::vertical([
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ]);

        let [upper_item_list_area, lower_item_list_area] =
            vertical.areas(rest_area);

        self.render_title(header_area, buf);
        self.render_lv2_list_selected(upper_item_list_area, buf);
        self.render_port_controls(lower_item_list_area, buf);
        self.render_footer(footer_area, buf);
    }

    /// Ask mod-host what the value is for this port
    fn _get_port_value(&mut self, index: usize, symbol: &str) {
        let cmd = format!("param_get {index} {symbol}");
        self.send_mh_cmd(cmd.as_str());
    }
}

/// The main method got making the UI
impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.app_view_state {
            AppViewState::Lv2SaveName => self.render_save_name(area, buf),
            AppViewState::List => self.render_list(area, buf),
            AppViewState::Command => self.render_selected_lv2(area, buf),
        };
    }
}

impl App<'_> {
    fn render_lv2(&self, lv2: &Lv2) -> String {
        format!("{lv2}")
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Qzn3t Lv2 Control")
            .centered()
            .render(area, buf);
    }

    fn render_lv2_list_selected(&mut self, area: Rect, buf: &mut Buffer) {
        // We create two blocks, one is for the header (outer) and the other is for list (inner).
        let outer_block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(HEADER_BG)
            .title("LV2 Simulators")
            .title_alignment(Alignment::Center);
        let inner_block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(NORMAL_ROW_COLOR);

        // We get the inner area from outer_block. We'll use this area later to render the table.
        let outer_area = area;
        let inner_area = outer_block.inner(outer_area);

        // We can render the header in outer_area.
        outer_block.render(outer_area, buf);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .lv2_stateful_list
            .items
            .iter()
            .enumerate()
            .filter(|&l| l.1.status == Status::Loaded)
            .map(|(i, lv2_item)| Self::sim_to_static_list_item(lv2_item, i))
            .collect();
        // Create a List from all list items and highlight the currently selected one
        let items2 = List::new(items)
            .block(inner_block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We can now render the item list
        // (look careful we are using StatefulWidget's render.)
        // ratatui::widgets::StatefulWidget::render as stateful_render
        StatefulWidget::render(
            items2,
            inner_area,
            buf,
            &mut self.lv2_loaded_list.state,
        );
    }

    /// Make a ListItem for App::lv2_loaded_list
    fn sim_to_static_list_item(sim: &Lv2Simulator, index: usize) -> ListItem {
        let bg_color = match index % 2 {
            0 => NORMAL_ROW_COLOR,
            _ => ALT_ROW_COLOR,
        };
        let line = Line::styled(
            format!("{} effect_{} ", sim.name, sim.mh_id,),
            STATIC_TEXT_FG,
        );

        ListItem::new(line).bg(bg_color)
    }

    /// Make a list item for App::lv2_stateful_list
    fn sim_lv2_list_item(sim: &Lv2Simulator, index: usize) -> ListItem {
        let bg_color = match index % 2 {
            0 => NORMAL_ROW_COLOR,
            _ => ALT_ROW_COLOR,
        };
        // SELECTED_TEXT_FG
        let line = match sim.status {
            Status::Loaded => Line::styled(
                format!(" ☐ {:>3} {}", sim.mh_id, sim.name),
                SELECTED_TEXT_FG,
            ),
            Status::Unloaded => Line::styled(
                format!(" ✓ {:>3} {}", sim.mh_id, sim.name),
                (COMPLETED_TEXT_COLOR, bg_color),
            ),
            Status::Pending => Line::styled(
                format!(" {:>3} {} ", sim.mh_id, sim.name),
                (PENDING_TEXT_COLOR, bg_color),
            ),
        };
        ListItem::new(line).bg(bg_color)
    }

    /// A list of all LV2 Simulators known.
    /// `area`: Where to draw the list
    /// `buffer`: ??? FI2K
    fn render_lv2_list(&mut self, area: Rect, buf: &mut Buffer) {
        // We create two blocks, one is for the header (outer) and the other is for list (inner).
        let outer_block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(HEADER_BG)
            .title("LV2 Simulators")
            .title_alignment(Alignment::Center);
        let inner_block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(NORMAL_ROW_COLOR);

        // We get the inner area from outer_block. We'll use this area later to render the table.
        let outer_area = area;
        let inner_area = outer_block.inner(outer_area);

        // We can render the header in outer_area.
        outer_block.render(outer_area, buf);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .lv2_stateful_list
            .items
            .iter()
            .enumerate()
            .map(|(i, simulator_lv2)| Self::sim_lv2_list_item(simulator_lv2, i))
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(inner_block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We can now render the item list
        // (look careful we are using StatefulWidget's render.)
        // ratatui::widgets::StatefulWidget::render as stateful_render
        StatefulWidget::render(
            items,
            inner_area,
            buf,
            &mut self.lv2_stateful_list.state,
        );
    }

    /// Render the details from the selected list's selected item
    /// `area` is the area it is drawn in
    fn render_details(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.get_stateful_list().state.selected() {
            self.render_lv2(&self.mod_host_controller.simulators.as_slice()[i])
        } else {
            "Nothing to see here...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let outer_info_block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(HEADER_BG)
            .title("Details")
            .title_alignment(Alignment::Center);
        let inner_info_block = Block::default()
            .borders(Borders::NONE)
            .bg(NORMAL_ROW_COLOR)
            .padding(Padding::horizontal(1));

        // This is a similar process to what we did for list. outer_info_area will be used for
        // header inner_info_area will be used for the list info.
        let outer_info_area = area;
        let inner_info_area = outer_info_block.inner(outer_info_area);

        // We can render the header. Inner info will be rendered later
        outer_info_block.render(outer_info_area, buf);

        let info_paragraph = Paragraph::new(info)
            .block(inner_info_block)
            .fg(TEXT_COLOR)
            .wrap(Wrap { trim: false });

        // We can now render the item info
        info_paragraph.render(inner_info_area, buf);
    }

    // In screen 2 (F2) render the control details for the selected
    // LV2 simulator `area` is the screen real-estate that can be used
    fn render_port_controls(&mut self, area: Rect, buf: &mut Buffer) {
        // We show the list item's info under the list in this paragraph
        let outer_info_block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(HEADER_BG)
            .title("Port Controls")
            .bold()
            .title_alignment(Alignment::Center);

        let inner_info_area: Rect = outer_info_block.inner(area);

        // Render the controls into `inner_info_area`
        self.scroll_bar_state =
            ScrollbarState::new((self.control_ports.len()) * ITEM_HEIGHT);
        // Get the table widget
        let table: Table = port_table(&self.control_ports, &self.port_values); //.to_vec());
        ratatui::widgets::StatefulWidget::render(
            table,
            inner_info_area,
            buf,
            &mut self.table_state,
        );
        ratatui::widgets::StatefulWidget::render(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            inner_info_area,
            buf,
            &mut self.scroll_bar_state,
        );
        // table.render(inner_info_area, buf);

        // We can render the header. Inner info will be rendered later
        outer_info_block.render(area, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        match self.app_view_state {
			 AppViewState::Lv2SaveName => Paragraph::new("Enter a new name to save this simulator's state\nExisting name to load a simulator"),
          AppViewState::List => Paragraph::new(
            "Use ↓↑ to select simulators <enter> to load/unload, \ng/G to go top/bottom.",
         ),
         AppViewState::Command => Paragraph::new(
             "Use ↓↑ to move between simulators Enter to load \n \
				  n/p move between ports.  ← decrease → increase port value"
         ),
      }
      .centered()
      .render(area, buf);
    }
}

impl Lv2StatefulList {
    fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if (i + 1) >= self.items.len() {
                    i // `next` at bottom of list has no effect
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0 // `previous` at top of list has no effect
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }
}
