use crate::lv2::Lv2;
use crate::run_executable::rem_trail_0;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::Result;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[derive(Debug)]
/// Interface to mod-host
pub struct ModHostController {
   pub simulators: Vec<Lv2>,
   pub mod_host_th: thread::JoinHandle<()>,
   // pub data_th: thread::JoinHandle<()>,
   pub input_tx: Sender<Vec<u8>>, // Send data to mod-host
   pub output_rx: Receiver<Vec<u8>>, // Get data from mod-host

   /// The last command sent to mod-host.  It is command orientated
   /// so a "resp..." from mod-host refers to the last command sent.
   /// This programme is asynchronous, so a command is sent, and
   /// later a response is received.  This allows the two to be
   /// connected.  When a response is received set this back to None.
   pub last_mh_command: Option<String>,

   /// Commands are queued when they arrive.  They are sent in the
   /// order they are received.
   pub mh_command_queue: VecDeque<String>,
}

impl ModHostController {
   // /// Set a value for a port
   pub fn set_port_value(
      &mut self,
      _instance_number: usize,
      _symbol: &str,
      _value: &str,
   ) {
   }
   // pub fn set_port_value(&mut self, instance_number:usize, symbol: &str, value: &str){
   // 	self.simulators.iter_mut().find(|s|
   // }

   /// Get a response from mod-host if one is available.  Will block
   /// until some is available.  
   pub fn get_data(&self) -> Result<String> {
      let resp = match self.output_rx.recv() {
         Ok(t) => t,
         Err(err) => {
            return Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
         }
      };

      let resp = rem_trail_0(resp);
      match String::from_utf8(resp) {
         Ok(s) => Ok(s),
         Err(err) => {
            Err(io::Error::new(io::ErrorKind::InvalidData, err.to_string()))
         }
      }
   }

   /// Get a response from mod-host if one is available.  Will not block
   /// and returns Ok(None) if no data availale
   pub fn try_get_data(&self) -> Result<Option<String>> {
      match self.output_rx.try_recv() {
         Ok(t) => {
            // Got some data
            let resp = rem_trail_0(t);
            match String::from_utf8(resp) {
               Ok(s) => Ok(Some(s)),
               Err(err) => Err(io::Error::new(
                  io::ErrorKind::InvalidData,
                  err.to_string(),
               )),
            }
         }
         Err(err) => match err {
            // No data available
            TryRecvError::Empty => Ok(None),

            // Something bad
            TryRecvError::Disconnected => {
               Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
            }
         },
      }
   }

   /// Return `Lv2` by URL
   pub fn get_lv2_by_url(&mut self, url: &str) -> Option<&Lv2> {
      self.simulators.iter().find(|l| l.url == url)
   }
   pub fn get_lv2_by_url_mut(&mut self, url: &str) -> Option<&mut Lv2> {
      self.simulators.iter_mut().find(|l| l.url == url)
   }

   pub fn translate_error_code(error: isize) -> String {
      match error {
         -1 => "ERR_INSTANCE_INVALID".to_string(),
         -2 => "ERR_INSTANCE_ALREADY_EXISTS".to_string(),
         -3 => "ERR_INSTANCE_NON_EXISTS".to_string(),
         -4 => "ERR_INSTANCE_UNLICENSED".to_string(),
         -101 => "ERR_LV2_INVALID_URI".to_string(),
         -102 => "ERR_LV2_INSTANTIATION".to_string(),
         -103 => "ERR_LV2_INVALID_PARAM_SYMBOL".to_string(),
         -104 => "ERR_LV2_INVALID_PRESET_URI".to_string(),
         -105 => "ERR_LV2_CANT_LOAD_STATE".to_string(),
         -201 => "ERR_JACK_CLIENT_CREATION".to_string(),
         -202 => "ERR_JACK_CLIENT_ACTIVATION".to_string(),
         -203 => "ERR_JACK_CLIENT_DEACTIVATION".to_string(),
         -204 => "ERR_JACK_PORT_REGISTER".to_string(),
         -205 => "ERR_JACK_PORT_CONNECTION".to_string(),
         -206 => "ERR_JACK_PORT_DISCONNECTION".to_string(),
         -301 => "ERR_ASSIGNMENT_ALREADY_EXISTS".to_string(),
         -302 => "ERR_ASSIGNMENT_INVALID_OP".to_string(),
         -303 => "ERR_ASSIGNMENT_LIST_FULL".to_string(),
         -304 => "ERR_ASSIGNMENT_FAILED".to_string(),
         -401 => "ERR_CONTROL_CHAIN_UNAVAILABLE".to_string(),
         -402 => "ERR_LINK_UNAVAILABLE".to_string(),
         -901 => "ERR_MEMORY_ALLOCATION".to_string(),
         -902 => "ERR_INVALID_OPERATION".to_string(),
         _ => format!("Unknown error code: {error}"),
      }
   }

   /// Queue a command to send to mod-host
   pub fn send_mh_cmd(&mut self, cmd: &str) {
      self.mh_command_queue.push_back(cmd.to_string());
   }

   /// Check for redundant commands in command queue.
   fn reduce_queue(&mut self) {
      // Start at the far end of the queue.  A command there that
      // sets a parameter means that any earlier commands (in thye
      // queue) that set the same parameter will be over ridden so
      // no point keeping them

      // Put the commands here that can be removed from the rest of
      // the queue
      let mut commands: HashSet<String> = HashSet::new();

      // This is the new queue without redundant commands
      let mut new_queue: VecDeque<String> = VecDeque::new();

      let cmd_iter = self.mh_command_queue.iter_mut().rev();
      for c in cmd_iter {
         if &c[.."param_set".len()] == "param_set" {
            // param_set <instance_number> <param_symbol> <param_value>
            //     e.g.: param_set 0 gain 2.50
            let cmd_end = c
               .chars()
               .rev()
               .position(|c| c.is_whitespace())
               .expect("reduce_queue: Find end of param_set");
            let cmd_key = &c[..cmd_end];
            if commands.contains(cmd_key) {
               // Do not process this
               eprintln!("INFO: reduce_queue - delete: {c}");
               continue;
            }
            commands.insert(cmd_key.to_string());
            new_queue.push_back(c.to_string());
            continue;
         }

         // Default is to keep command
         new_queue.push_back(c.to_string());
      }
      self.mh_command_queue = new_queue;
   }

   /// Called from the event loop to send a message to mod-host
   pub fn pump_mh_queue(&mut self) {
      self.reduce_queue();
      if self.last_mh_command.is_none() && !self.mh_command_queue.is_empty() {
         // Safe because queue is not empty
         let cmd = self.mh_command_queue.pop_front().unwrap();

         eprintln!("CMD: {cmd}");
         self.last_mh_command = Some(cmd.trim().to_string());
         self
            .input_tx
            .send(cmd.as_bytes().to_vec())
            .expect("Send to mod-host");
      }
   }
   pub fn get_queued_count(&self) -> usize {
      self.mh_command_queue.len()
   }
   pub fn get_last_mh_command(&self) -> Option<String> {
      self.last_mh_command.clone()
   }
   pub fn set_last_mh_command(&mut self, cmd: Option<String>) {
      self.last_mh_command = cmd;
   }
}
