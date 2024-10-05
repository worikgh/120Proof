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
use crate::lv2::Lv2;
use crate::lv2_simulator::Lv2Simulator;
use crate::lv2_simulator::Status;
use crate::lv2_stateful_list::Lv2StatefulList;
use crate::mod_host_controller::ModHostController;
use crate::port::Port;
use crate::port::PortType;
use crate::port_table::port_table;
use color_eyre::config::HookBuilder;
use crossterm::{
   event::{self, Event, KeyCode},
   terminal::{
      disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
      LeaveAlternateScreen,
   },
   ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::thread;
use std::time::{Duration, Instant};
use std::{error::Error, io, io::stdout};

/// Used by the scroll bar for LV2 Controls (F2).  So far the scroll
/// bars do nothing at all
const ITEM_HEIGHT: usize = 1;

/// Encodes whether a port value is being incremented `up` or
/// decremented `down` when adjusting the port value.
/// This allows the same code do be used for both cases
enum PortAdj {
   Up,
   Down,
}

#[derive(Debug, PartialEq, Eq)]
enum AppViewState {
   List,    // Listing all simulators
   Command, // Interacting with mod-host
}

/// This struct holds the current state of the app.
pub struct App<'a> {
   // Data from mod-host
   buffer: String,

   /// JACK audio Connections
   jack_connections: HashSet<String>,

   mod_host_controller: &'a mut ModHostController,

   /// Maintain the view for the first screen, and the main data
   /// model for the simulators.  ?? Should the stateful model of
   /// LV2s (state is loaded/unloaded and connections) be in
   /// `ModHostController` ??
   lv2_stateful_list: Lv2StatefulList,

   /// Maintain the view for the second screen of loaded simulators
   lv2_loaded_list: Lv2StatefulList,

   /// The current view
   app_view_state: AppViewState,

   /// Internal state to prevent complaining too much about
   /// unrecogised responses from mod-host
   unrecognised_resp: HashSet<String>,

   /// The Ports in the current view and the scrolling table they are
   /// displayed in
   ports: Vec<Port>,
   table_state: TableState,
   scroll_bar_state: ScrollbarState,

   // /// The last command sent to mod-host.  It is command orientated
   // /// so a "resp..." from mod-host refers to the last command sent.
   // /// This programme is asynchronous, so a command is sent, and
   // /// later a response is received.  This allows the two to be
   // /// connected.  When a response is received set this back to None.
   // last_mh_command: Option<String>,

   // /// Commands are queued when they arrive.  They are sent in the
   // /// order they are received.
   // mh_command_queue: VecDeque<String>,
   /// Store the last status output so do not thrash status reporting
   /// mechanism (eprintln! as I write) with repeated status messages
   status: Option<String>,

   /// For debugging state
   _dbg_s: String,
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

      App {
         jack_connections: HashSet::new(),
         buffer: "".to_string(),
         app_view_state: AppViewState::List,
         mod_host_controller,
         lv2_loaded_list: Lv2StatefulList::empty(),
         lv2_stateful_list: Lv2StatefulList::new(&types),
         unrecognised_resp: HashSet::new(),

         table_state: TableState::default().with_selected(0),
         scroll_bar_state: ScrollbarState::default(),
         ports: vec![],

         // last_mh_command: None,
         // mh_command_queue: VecDeque::new(),
         status: None,
         _dbg_s: "".to_string(),
      }
   }

   fn status_string(&self) -> String {
      format!(
         "{:?} queued cmd# {} Last Command: {:?}",
         self.app_view_state,
         self.mod_host_controller.get_queued_count(),
         self.mod_host_controller.get_last_mh_command()
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

   /// Changes the status of the selected list item.  
   fn change_status(&mut self) {
      if self.get_stateful_list().items.is_empty() {
         // Nothing to do
         return;
      }
      match self.app_view_state {
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

                        let cmd = format!("add {} {}\n", lv2.url, lv2.mh_id);
                        self.send_mh_cmd(cmd.as_str());

                        // TODO:  Move this into the handler for received messages
                        self.lv2_loaded_list.items.insert(
                           if i > self.lv2_loaded_list.items.len() {
                              self.lv2_loaded_list.items.len()
                           } else {
                              i
                           },
                           lv2,
                        );
                        Status::Pending
                     }

                     // Set status of Lv2 to Unloaded.  Remove from the
                     // list of loaded simulators
                     Status::Loaded => {
                        if self.lv2_loaded_list.items.iter().any(|x| {
                           x.url == self.lv2_stateful_list.items[i].url
                        }) {
                           let lv2: Lv2Simulator =
                              self.lv2_stateful_list.items[i].clone();
                           let cmd = format!("remove {}\n", lv2.mh_id);

                           self.send_mh_cmd(cmd.as_str());

                           Status::Unloaded
                        } else {
                           panic!("A loaded LV2 was not om loaded_list.  {i}");
                        }
                     }
                     Status::Pending => Status::Pending,
                  }
            }
         }
         AppViewState::Command => {
            // Showing and editing the Port values for a LV2.  Change the LV2 simulator
            // TODO: Optimisation - check the same LV2 is not selected twice
            if let Some(idx) = self.get_stateful_list_mut().state.selected() {
               // Connect the selected effect to system in/out
               eprintln!("INFO change_status AppViewState::Command idx: {idx}");
               let mh_id = self.get_stateful_list().items[idx].mh_id;

               eprintln!(
                  "INFO Effect: effect_{idx} dispay ports.  mh_id: {mh_id}"
               );

               //  Disconnect any existing connections.  This
               //  connects one, and only one, LV2
               let disconnect_cmds = self
                  .jack_connections
                  .iter()
                  .map(|s| format!("disconnect {s}"))
                  .collect::<Vec<String>>();
               eprintln!("INFO Disconnect commands: {disconnect_cmds:?}");
               let control_commands: Vec<String>;
               let input_commands: Vec<String>;
               let output_commands: Vec<String>;
               {
                  let lv_url = self.get_stateful_list().items[idx].url.clone();
                  let lv2 = match self
                     .mod_host_controller
                     .get_lv2_by_url(lv_url.as_str())
                  {
                     Some(l) => l,
                     None => panic!("Getting Lv2 by url"),
                  };
                  self.ports = lv2
                     .ports
                     .iter()
                     .filter(|&p| {
                        p.types
                           .iter()
                           .any(|t| matches!(t, PortType::Control(_)))
                           && p.types.contains(&PortType::Input)
                     })
                     .cloned()
                     .collect::<Vec<Port>>();
                  control_commands = self
                     .ports
                     .iter()
                     .filter(|&p| {
                        p.types
                           .iter()
                           .any(|t| matches!(t, PortType::Control(_)))
                           && p.types.contains(&PortType::Input)
                           && p.value.is_none()
                     })
                     .map(|p| format!("param_get {mh_id} {}", p.symbol))
                     .collect::<Vec<String>>();

                  let output_ports = lv2
                     .ports
                     .iter()
                     .filter(|p| {
                        p.types.iter().any(|t| t == &PortType::Output)
                           && p.types.iter().any(|t| t == &PortType::Audio)
                     })
                     .collect::<Vec<&Port>>();
                  let input_ports = lv2
                     .ports
                     .iter()
                     .filter(|p| {
                        p.types.iter().any(|t| t == &PortType::Input)
                           && p.types.iter().any(|t| t == &PortType::Audio)
                     })
                     .collect::<Vec<&Port>>();
                  let mut i = 1;
                  input_commands = input_ports
                     .iter()
                     .map(|p| {
                        let lhs = format!("system:capture_{i}");
                        let rhs =
                           format!("effect_{mh_id}:{}", p.symbol.as_str());
                        i += 1;
                        format!("connect {lhs} {rhs}")
                     })
                     .collect();
                  let mut i = 1;
                  output_commands = output_ports
                     .iter()
                     .map(|p| {
                        let lhs =
                           format!("effect_{mh_id}:{}", p.symbol.as_str());
                        let rhs = format!("system:playback_{i}");
                        i += 1;
                        format!("connect {lhs} {rhs}")
                     })
                     .collect();
               }
               eprintln!(
                  "INFO: Issuing {} commands to mod-host",
                  disconnect_cmds.len()
                     + input_commands.len()
                     + output_commands.len()
                     + control_commands.len()
               );
               for cmd in disconnect_cmds.iter() {
                  self.mod_host_controller.send_mh_cmd(cmd.as_str());
               }
               for cmd in input_commands.iter() {
                  self.mod_host_controller.send_mh_cmd(cmd.as_str());
               }
               for cmd in output_commands.iter() {
                  self.mod_host_controller.send_mh_cmd(cmd.as_str());
               }
               for cmd in control_commands.iter() {
                  self.mod_host_controller.send_mh_cmd(cmd.as_str());
               }
               eprintln!("INFO: Commands all sent");
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

      app._run(terminal).expect("Calling _run");

      restore_terminal()?;

      Ok(())
   }

   /// Set a status line
   fn set_status(&self, status: &str) {
      // No actual status yet
      eprintln!("3 INFO Status: {status}");
   }

   /// Handle a response from mod-host that starts with "resp ".  It
   /// is a response to a command, so what happens here is dependant
   /// on that command    
   /// resp status [value]
   fn process_resp(&mut self, response: &str) {
      // Can only get a "resp " from mod-host after a command has been sent
      let last_mh_command = match self.mod_host_controller.get_last_mh_command()
      {
         Some(s) => s.trim().to_string(),
         None => panic!(
            "Handeling 'resp' response but there is no `last_mh_command`"
         ),
      };

      // Get the first word as a slice
      let sp: usize = last_mh_command
         .chars()
         .position(|x| x.is_whitespace())
         .expect("No space in resp string");
      let fw_cmd = &last_mh_command[0..sp];

      match fw_cmd {
         "add" => {
            // Adding an LV2.  Get the instance number from the
            // command, the instance number that the cammand was
            // addressed to

            let sp = last_mh_command.rfind(' ').unwrap_or_else(|| {
               panic!("Malformed command: '{last_mh_command}'")
            });
            let instance_number = last_mh_command[sp..].trim().parse::<usize>().unwrap_or_else(|_| {
                    panic!(
                        "No instance number at end of add command: '{last_mh_command}'  sp: {sp} => '{}'",
			&last_mh_command[sp..]
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
            let n = response[5..]
               .parse::<isize>()
               .expect("No instance number at end of response");

            if n >= 0 {
               // `n` is the instance_number of the simulator
               assert!(n as usize == instance_number);

               item.status = Status::Loaded;
            } else {
               // Error code
               let errno = n;
               eprintln!(
                  "ERR: {errno}.  Command: {:?}: {}",
                  self.mod_host_controller.get_last_mh_command(),
                  ModHostController::translate_error_code(n)
               );
               item.status = Status::Unloaded;
            }
         }
         "param_get" => {
            // Got the current value of a Port.  Get the symbol for the port from the command
            let q = Self::get_instance_symbol_res(
               last_mh_command.as_str(),
               response,
            );
            let instance_number = q.0;
            let symbol = q.1;
            let n = q.2;
            match n.cmp(&0) {
               Ordering::Less => {
                  eprintln!(
                     "ERR: {n}.  Command: {:?}: {}",
                     self.mod_host_controller.get_last_mh_command(),
                     ModHostController::translate_error_code(n)
                  );
               }
               Ordering::Equal => {
                  // Got a value.  Cache it in the port
                  let lsp = response.len()
                     - response
                        .chars()
                        .rev()
                        .position(|c| c.is_whitespace())
                        .unwrap_or(0);
                  let value = response[lsp..].trim();
                  self.update_port(instance_number, symbol, value);
               }
               Ordering::Greater => panic!("Bad n {n} from mod-host in resp"),
            };
         }
         "param_set" => {
            // E.g: "param_set 1 Gain 0"
            let q = Self::get_instance_symbol_res(
               last_mh_command.as_str(),
               response,
            );
            let instance_number = q.0;
            let symbol = q.1;
            let n = q.2;
            match n.cmp(&0) {
               Ordering::Less => {
                  eprintln!(
                     "ERR: {n}.  Command: {:?}: {}",
                     self.mod_host_controller.get_last_mh_command(),
                     ModHostController::translate_error_code(n)
                  );
               }
               Ordering::Equal => {
                  // Set the value in the LV2, update our records
                  // Get the value from the command
                  let sp = last_mh_command.len()
                     - last_mh_command
                        .chars()
                        .rev()
                        .position(|c| c.is_whitespace())
                        .unwrap_or(0);
                  let value = last_mh_command.as_str()[sp..].trim();
                  self.update_port(instance_number, symbol, value);
               }
               Ordering::Greater => panic!("Bad n {n} from mod-host in resp"),
            }
         }
         "remove" => {
            // Removing an LV2.  Get the instance number from the command
            let sp = last_mh_command.rfind(' ').unwrap_or_else(|| {
               panic!("Malformed command: '{last_mh_command}'")
            });
            let instance_number = last_mh_command[sp..].trim().parse::<usize>().unwrap_or_else(|_| {
                    panic!(
                        "No instance number at end of add command: '{last_mh_command}'  sp: {sp} => '{}'",
			&last_mh_command[sp..]
                    )
                });

            // Get response.  If 0, all is good.  Otherwise there
            // is an error.  Leave item pending
            if let Ok(n) = response[5..].parse::<isize>() {
               match n.cmp(&0) {
                  Ordering::Equal => {
                     self
                        .lv2_stateful_list
                        .items
                        .iter_mut()
                        .find(|x| x.mh_id == instance_number)
                        .expect("Cannot find LV2 instance: {instance_number}")
                        .status = Status::Unloaded
                  }
                  Ordering::Greater => {
                     eprintln!("Bad response.  n > 0: {response}")
                  }
                  Ordering::Less => eprintln!(
                     "M-H Err: {n} => {}",
                     ModHostController::translate_error_code(n)
                  ),
               };
            } else {
               eprintln!("ERR Bad resp: {response}");
            }
            self.mod_host_controller.set_last_mh_command(None);
         }
         "connect" => {
            // A connection was established
            // TODO:  Record connections in model data
            let jacks = &last_mh_command.as_str()[sp + 1..];
            self.jack_connections.insert(jacks.to_string());
            eprintln!("INFO jacks: {jacks}");
         }
         "disconnect" => {
            let jacks = &last_mh_command.as_str()[sp + 1..];
            if !self.jack_connections.remove(jacks) {
               panic!("Failed to remove {jacks} from connections");
            }
         }
         _ => panic!("Unknown command: {last_mh_command}"),
      };

      // Having handled the command, one way or another, delete it
      self.mod_host_controller.set_last_mh_command(None);
   }

   /// Process data coming from mod-host.  Line orientated and asynchronous
   fn process_buffer(&mut self) {
      // If there is no '\n' in buffer, do not process it, leave it
      // till next time.  But process all lines that are available
      while let Some(s) = self.buffer.as_str().find('\n') {
         // There is a line available
         let r = self.buffer.as_str()[0..s].trim().to_string();
         if !r.is_empty() {
            // Skip blank lines.
            eprintln!("INFO m-h: {r}");
            if r == "mod-host>" || r == "using block size: 1024" {
            } else if r.len() > 5 && &r.as_str()[0..5] == "resp " {
               self.process_resp(r.as_str());
            } else {
               match &self.mod_host_controller.get_last_mh_command() {
                  Some(s) => {
                     if s.trim() == r.trim()
                        || format!("mod-host> {}", s.trim()).as_str()
                           == r.trim()
                     {
                        // All good mod-host repeats back commands
                        // Command is not complete yet
                     } else {
                        eprintln!("ERR: '{s}': Bad response: '{r}'");
                     }
                  }
                  None => {
                     if self.unrecognised_resp.insert(r.clone()) {
                        eprintln!("INFO Unrecognised: {r}")
                     }
                  }
               };
            }
         }
         self.buffer = if s < self.buffer.len() {
            self.buffer.as_str()[(s + 1)..].to_string()
         } else {
            "".to_string()
         };
      }
   }

   /// Set a value to the port named by `symbol` to the LV2 with `instance_number`
   fn update_port(
      &mut self,
      instance_number: usize,
      symbol: &str,
      value: &str,
   ) {
      // let idx:usize = self.get_stateful_list_mut().state.selected().expect("Get selected");
      eprintln!("INFO: update_port {instance_number} {symbol} {value}");
      let lv2_name = self
         .get_stateful_list_mut()
         .items
         .iter_mut()
         .find(|x| x.mh_id == instance_number)
         .expect("Find LV2 with mh_id")
         .name
         .clone();

      self
         .mod_host_controller
         .simulators
         .iter_mut()
         .find(|l| l.name == lv2_name)
         .expect("Find Lv2 by name!")
         .ports
         .iter_mut()
         .find(|p| p.symbol == symbol)
         .expect("Finding port by symbol")
         .value = Some(value.to_string());

      // Update cached version
      for p in self.ports.iter_mut() {
         if p.symbol == symbol {
            p.value = Some(value.to_string());
         }
      }
   }

   /// When responding to a param_get or param_set extract the
   /// instance number and the symbol from the last command
   fn get_instance_symbol_res<'a>(
      last_mh_command: &'a str,
      response: &'a str,
   ) -> (usize, &'a str, isize) {
      // Got the current value of a Port.  Get the symbol for the port from the command
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

      // Got instance and symbol from command

      // Get the status and the value from the response
      let r = &response[5..];
      let sp: usize = r.chars().position(|x| x.is_whitespace()).unwrap_or(
         // No whitespace till end of string
         r.len(),
      );
      let n = r[..sp].trim().parse::<isize>().expect(
         "Malformed response to param_get: Instance number \
			    invalid. {response}",
      );
      (instance_number, symbol, n)
   }

   fn handle_port_adj(&mut self, adj: PortAdj) {
      {
         if let Some(url) = self.get_stateful_list().get_selected_url() {
            if let Some(port_table_row) = self.table_state.selected() {
               // Got index to a Port  Get its name
               let n = self.ports[port_table_row].name.as_str();
               eprintln!("INFO handle_port_adj url: {url} idx: {port_table_row} name: {n}");

               // Get a mutable reference
               // to the LV2 simulator
               // whoes port is to be
               // adjusted
               let l = self
                  .mod_host_controller
                  .get_lv2_by_url_mut(&url)
                  .expect("Cannot get LV2 by URL");

               // Got LV2 and name of port, get a mutable reference
               let p = l
                  .ports
                  .iter_mut()
                  .find(|p| p.name == n)
                  .expect("Find port by name");

               // Values are encoded as String (by accident TODO Fix it)
               let mdml = p
                  .get_min_def_max()
                  .expect("Getting min, default, max, log adjusting port");

               let v = if let Some(ref v) = p.value {
                  v.parse::<f64>().expect("Translate value to f64")
               } else {
                  panic!("No value for port in adj");
               };

               let v = if mdml.3 {
                  // Logarithmic
                  // Make linear
                  let min_ln = mdml.0.ln();
                  let max_ln = mdml.2.ln();
                  let step_size_ln = (max_ln - min_ln) / 127.0_f64;

                  // Adjust value in linear scale then convert back
                  let vn = v.ln();
                  match adj {
                     PortAdj::Down => (vn - step_size_ln).exp(),
                     PortAdj::Up => (vn + step_size_ln).exp(),
                  }
               } else {
                  // Linear
                  v + (mdml.2 - mdml.0) / 127.0_f64
                     * match adj {
                        PortAdj::Down => -1.0_f64,
                        PortAdj::Up => 1.0_f64,
                     }
               };

               // Adjust value so within bounds
               let v = if v < mdml.0 {
                  0.0_f64
               } else if v > mdml.2 {
                  mdml.2
               } else {
                  v
               };
               p.value = Some(format!("{v}"));
               let symbol = p.symbol.clone();

               let instance_number = self
                  .get_stateful_list()
                  .get_selected_mh_id()
                  .expect("handle_port_adj: A selected item");
               let cmd =
                  format!("param_set {} {} {v}", instance_number, symbol);
               self.send_mh_cmd(cmd.as_str());
            }
         }
      }
   }

   /// The main body of the App
   fn _run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
      // init_error_hooks().expect("App::run error hooks");

      let target_fps = 60; // 400 is about the limit on Raspberry Pi 5
      let frame_time = Duration::from_secs(1) / target_fps as u32;

      // Set this to false to make process exit on next loop
      let mut run = true;
      loop {
         let start_time = Instant::now();
         if !run {
            break;
         }

         // If queue has gotten too big, something has gone wrong.
         // if self.mod_host_controller.get_queued_count() > 100 {
         //     eprintln!("ERR Aborting.  Queue too large {}", self.status_string());
         //     break;
         // }

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

         if event::poll(Duration::from_secs(0)).expect("Polling for event") {
            match event::read() {
               Ok(Event::Key(key)) => {
                  use KeyCode::*;
                  match key.code {
                     Left => {
                        // In Port Control window decrease value of
                        // port by one unit
                        if self.app_view_state == AppViewState::Command {
                           self.handle_port_adj(PortAdj::Down);
                        }
                     }
                     Right => {
                        // In Port Control window increase value of
                        // port by one unit
                        if self.app_view_state == AppViewState::Command {
                           self.handle_port_adj(PortAdj::Up);
                        }
                     }
                     Char('q') | Esc => {
                        self.send_mh_cmd("quit");
                        // Move this to handler of data from mod-host?
                        //return Ok(());
                        run = false;
                     }
                     Char('u') => self.get_stateful_list_mut().unselect(),
                     Down => self.get_stateful_list_mut().next(),
                     Up => self.get_stateful_list_mut().previous(),
                     Enter => self.change_status(),
                     Char('g') => self.go_top(),
                     Char('G') => self.go_bottom(),

                     Char('n') => {
                        // In LV2 Control view (F2) move down
                        // on Port in Port display
                        if self.app_view_state == AppViewState::Command {
                           // In LV2 Control view (F2) move down a port
                           let i = match self.table_state.selected() {
                              Some(i) => {
                                 if i >= self.ports.len() - 1 {
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
                        // In LV2 Control view (F2) move down
                        // on Port in Port display
                        if self.app_view_state == AppViewState::Command {
                           // In LV2 Control view (F2) move down
                           let i = match self.table_state.selected() {
                              Some(i) => {
                                 if i == 0 {
                                    self.ports.len() - 1
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
               Ok(Event::Resize(_, _)) => (),
               Err(err) => panic!("{err}: Reading event"),
               x => panic!("Error reading event: {x:?}"),
            };
         }

         // Send data to mod-host if it is enqueued
         self.pump_mh_queue();

         // Is there any data from mod-host
         if let Ok(Some(data)) = self.mod_host_controller.try_get_data() {
            self.buffer += data.as_str();
            self.process_buffer();
         }

         let elapsed_time = Instant::now() - start_time;
         if elapsed_time < frame_time {
            thread::sleep(frame_time - elapsed_time);
         } else {
            eprintln!("Timing error: {elapsed_time:?}/{frame_time:?}");
         }
      }
      Ok(())
   }

   fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
      terminal.draw(|f| f.render_widget(self, f.size()))?;
      Ok(())
   }

   /// Get the StateFulList that is currently in view
   fn get_stateful_list(&self) -> &Lv2StatefulList {
      match self.app_view_state {
         AppViewState::List => &self.lv2_stateful_list,
         AppViewState::Command => &self.lv2_loaded_list,
      }
   }

   /// Get a mutable reference to the StateFulList that is currently
   /// in view
   fn get_stateful_list_mut(&mut self) -> &mut Lv2StatefulList {
      match self.app_view_state {
         AppViewState::List => &mut self.lv2_stateful_list,
         AppViewState::Command => &mut self.lv2_loaded_list,
      }
   }

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
      let vertical = Layout::vertical([
         Constraint::Percentage(50),
         Constraint::Percentage(50),
      ]);
      let [upper_item_list_area, lower_item_list_area] =
         vertical.areas(rest_area);

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
      let lv2_simulators: Vec<&Lv2Simulator> =
         self.lv2_stateful_list.items.iter().collect();
      let items: Vec<ListItem> = lv2_simulators
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
         ScrollbarState::new((self.ports.len()) * ITEM_HEIGHT);
      // Get the table widget
      let table = port_table(&self.ports.to_vec());
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
         AppViewState::List => Paragraph::new(
            "Use ↓↑ to move, ← to unselect, → to change status, \
		 g/G to go top/bottom.\nAny other character to send instructions",
         ),
         AppViewState::Command => Paragraph::new(
            "Use ↓↑ to move, ← to unselect, → to change status.\n\
		 Any other characters fol to send instructions <Enter> to send",
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
