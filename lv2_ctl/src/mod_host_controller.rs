use crate::lv2::Lv2;
use crate::lv2::Lv2Datum;
use crate::lv2::Lv2Type;
use crate::port::ContinuousType;
use crate::port::ControlPortProperties;
use crate::port::Port;
use crate::port::PortType;
use crate::run_executable::rem_trail_0;
use crate::run_executable::run_executable;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::Result;
use std::sync::mpsc::channel;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[derive(Debug)]
pub enum ConDisconFlight {
    Connected,
    Disconnected,
    InFlight,
}

#[derive(Debug)]
/// Interface to mod-host
pub struct ModHostController {
    pub simulators: Vec<Lv2>,

    /// Encapsulate the `mod-host` process in a thread
    pub mod_host_th: thread::JoinHandle<()>,
    pub input_tx: Sender<Vec<u8>>, // Send data to mod-host
    pub output_rx: Receiver<Vec<u8>>, // Get data from mod-host

    /// Commands to be sent to `mod-host` are queued when they arrive.
    /// They are sent in the order they are received.
    pub mh_command_queue: VecDeque<String>,

    /// The commands sent to mod-host.  
    pub sent_commands: HashSet<String>,

    /// The last command as reported by mod-host
    pub resp_command: Option<String>,

    pub connections: HashMap<String, ConDisconFlight>,
}

impl ModHostController {
    /// `lines` defines the LV2 simulators on the host computer.  Uses
    /// the output of [serd](https://gitlab.com/drobilla/serd)
    pub fn get_lv2_controller(
        lines: impl Iterator<Item = Result<String>>,
        // lines: Lines<StdinLock>,
    ) -> Result<ModHostController> {
        let mut lv2_data: Vec<Lv2Datum> = vec![];
        let mut subject_store: HashMap<String, usize> = HashMap::new();
        let mut predicate_store: HashMap<String, usize> = HashMap::new();
        let mut object_store: HashMap<String, usize> = HashMap::new();

        let mut index_sbj = 0;
        for line in lines {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            //.map(|x| x.unwrap()) {
            // `line` is n three useful parts: Subject, Predicate, and Object
            // 1. The line upto the first space ' ' is the Subject
            // 2. The remainder of the line upto the next space is the Predicate
            // 3. The remainder of the line, except for the last two characters, is the Object
            // The Object can contain spaces.
            // The last two characters are " ."

            let mut split: Vec<&str> = line.as_str().split(' ').collect();
            assert!(split.len() > 2, "split: {split:?}\nline: {line}");
            let subject = split.remove(0).to_string();
            let predicate = split.remove(0).to_string();

            let _object = split.join(" ");
            if &_object[_object.len() - 2..] != " ." {
                panic!("Bad line: {line}");
            }
            let object = _object.as_str()[..(_object.len() - 2)].to_string();

            if !subject_store.contains_key(&subject) {
                // First time a subject is seen.

                subject_store.insert(subject.clone(), index_sbj);
                index_sbj += 1;
            }
            if !predicate_store.contains_key(&predicate) {
                predicate_store.insert(predicate.clone(), index_sbj);
                index_sbj += 1;
            }
            if !object_store.contains_key(&object) {
                object_store.insert(object.clone(), index_sbj);
                index_sbj += 1;
            }
            lv2_data.push(Lv2Datum {
                subject,
                predicate,
                object,
            });
        }

        // Keep track of which subjects have been processed
        let mut processed: HashSet<&String> = HashSet::new();

        // Keep track of the simulators to put into the result
        let mut simulators: Vec<Lv2> = vec![];

        for l in lv2_data.iter() {
            if &l.object == "<http://lv2plug.in/ns/lv2core#Plugin>" {
                // Examine this because it is a plugin.

                if !processed.insert(&l.subject) {
                    // This subject has been processed
                    continue;
                }
                // It is a plugin that has not been processed yet

                // Collect all data for this plugin identified by `subject`
                let plugin_data: Vec<&Lv2Datum> = lv2_data
                    .iter()
                    .filter(|lv| lv.subject == l.subject)
                    .collect();

                // Get name
                let plugin_name = plugin_data
                    .iter()
                    .filter(|lv| {
                        lv.predicate == "<http://usefulinc.com/ns/doap#name>"
                    })
                    .collect::<Vec<&&Lv2Datum>>()
                    .iter()
                    .fold("".to_string(), |a, &b| {
                        a + &b.object.as_str()[1..(b.object.len() - 1)]
                    });

                // Collect all types for this plugin.  Will probably be two or three
                let plugin_type: HashSet<Lv2Type> = plugin_data
                    .iter()
                    .filter(|lv| {
                        lv.predicate
                     == "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>"
                    })
                    .collect::<Vec<&&Lv2Datum>>()
                    .iter()
                    .map(|lv| {
                        let i = lv.object.find('#').unwrap();
                        let j = lv.object.rfind('>').unwrap();
                        match &lv.object.as_str()[(i + 1)..j] {
                            "Plugin" => Lv2Type::Plugin,
                            "ReverbPlugin" => Lv2Type::ReverbPlugin,
                            "ChorusPlugin" => Lv2Type::ChorusPlugin,
                            "FlangerPlugin" => Lv2Type::FlangerPlugin,
                            "PhaserPlugin" => Lv2Type::PhaserPlugin,
                            "WaveshaperPlugin" => Lv2Type::WaveshaperPlugin,
                            "FunctionalProperty" => Lv2Type::FunctionalProperty,
                            "SpectralPlugin" => Lv2Type::SpectralPlugin,
                            "LimiterPlugin" => Lv2Type::LimiterPlugin,
                            "AnalyserPlugin" => Lv2Type::AnalyserPlugnin,
                            "PitchPlugin" => Lv2Type::PitchPlugin,
                            "ObjectProperty" => Lv2Type::ObjectProperty,
                            "Property" => Lv2Type::Property,
                            "SpatialPlugin" => Lv2Type::SpatialPlugin,
                            "UtilityPlugin" => Lv2Type::UtilityPlugin,
                            "AmplifierPlugin" => Lv2Type::AmplifierPlugin,
                            "ExpanderPlugin" => Lv2Type::ExpanderPlugin,
                            "CompressorPlugin" => Lv2Type::CompressorPlugin,
                            "EQPlugin" => Lv2Type::EQPlugin,
                            "ModulatorPlugin" => Lv2Type::ModulatorPlugin,
                            "InstrumentPlugin" => Lv2Type::InstrumentPlugin,
                            "SimulatorPlugin" => Lv2Type::SimulatorPlugin,
                            "FilterPlugin" => Lv2Type::FilterPlugin,
                            "DelayPlugin" => Lv2Type::DelayPlugin,
                            "DistortionPlugin" => Lv2Type::DistortionPlugin,
                            "Class" => Lv2Type::Class,
                            "EnvelopePlugin" => Lv2Type::EnvelopePlugin,
                            "Project" => Lv2Type::Project,
                            x => Lv2Type::Other(x.to_string()),
                        }
                    })
                    .collect();

                // The ports.  These will be control ports and audio I/O ports
                let plugin_ports: Vec<Port>;
                {
                    // Collect all the subject names of ports for this simulator
                    let port_names: Vec<String> = plugin_data
                        .iter()
                        .filter(|lv| {
                            &lv.predicate
                                == "<http://lv2plug.in/ns/lv2core#port>"
                                && lv.subject == l.subject
                        })
                        .collect::<Vec<&&Lv2Datum>>()
                        .iter()
                        .map(|&&lv| lv)
                        .collect::<Vec<&Lv2Datum>>()
                        .iter()
                        .map(|p| p.object.clone())
                        .collect::<Vec<String>>();

                    // Process the ports Each port has a subject like
                    // `_:gx_zita_rev1b9`.  Usually about two dozen lines
                    // that describe a port

                    plugin_ports = port_names.iter()
			.map(|p| {
			    // :Vec<Vec<Port>> `p`

			    // `l` is the set of tripples that define this port
			    let plugin_data: Vec<&Lv2Datum> = lv2_data.iter().
				filter(|&x| &x.subject == p).collect();

			    let name: String = plugin_data
				.iter()
				.filter(|&l| l.predicate == "<http://lv2plug.in/ns/lv2core#name>")
				.collect::<Vec<&&Lv2Datum>>()
				.iter()
				.fold(String::new(), |a, b| {
				    // println!("Name: '{a}' + '{}'", b.object);
				    a + remove_quotes(b.object.as_str())
				});

			    let symbol: String = plugin_data
				.iter()
				.filter(|&l| l.predicate == "<http://lv2plug.in/ns/lv2core#symbol>")
				.collect::<Vec<&&Lv2Datum>>()
				.iter()
				.fold(String::new(), |a, b| {
				    // println!("Symbol: '{a}' + '{}'", b.object);
				    a + remove_quotes(b.object.as_str())
				});
			    let index: usize = plugin_data
				.iter()
				.filter(|&l| l.predicate == "<http://lv2plug.in/ns/lv2core#index>")
				.collect::<Vec<&&Lv2Datum>>()
				.iter()
				.fold(0_usize, |a, &b| {
				    let b2 = b.object.as_str()[1..].to_string();
				    let i = b2.find('"').expect("{b2}");
				    let b2 = b2.as_str()[..i].to_string();
				    let b2 = b2.as_str().parse::<usize>().expect("{b2} not a usize");
				    a + b2
				});

			    // Usually more than one type for a port
			    let types: Vec<PortType> = plugin_data
				.iter()
				.filter(|l| l.predicate == "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>")
				.collect::<Vec<&&Lv2Datum>>()
				.iter()
				.map(|&l| {
				    let i = l.object.find('#').unwrap();
				    let j = l.object.rfind('>').unwrap();
				    match &l.object.as_str()[(i + 1)..j] {
					// Input,
					// Output,
					// Control,
					// Audio,
					// Other(String),
					"InputPort" => PortType::Input,
					"ControlPort" => {
					    if let Some((min,
                                                 max,
                                                 default,
                                                 logarthmic,
                                                 scale,
                                                 tp)) =
                                                get_mmdls(&plugin_data, &lv2_data) {
					    PortType::Control(ControlPortProperties::new(
                                                min,
                                                max,
                                                default,
                                                logarthmic,
                                                scale.clone(),
                                                tp,))
					        }else{
                                                    PortType::Other(String::from("Unimplemented type(?)"))
                                                }
                                        },
					"OutputPort" => PortType::Output,
					"AudioPort" => PortType::Audio,
					"AtomPort" => PortType::AtomPort,
					x => PortType::Other(x.to_string()),
				    }
				})
				.collect();

			    Port {
				symbol,
				name,
				index,
				types,
				// value: None,
			    }
			})
			.collect::<Vec<Port>>();
                };

                let url =
                    l.subject.as_str()[1..(l.subject.len() - 1)].to_string();
                if !plugin_name.is_empty() {
                    let lv2 = Lv2 {
                        url,
                        types: plugin_type,
                        ports: plugin_ports,
                        name: plugin_name,
                    };
                    simulators.push(lv2);
                }
            }
        }

        eprintln!("Found {} simulators", simulators.len());

        // Run the mod-host sub-process
        // pub fn run_executable(path: &str, input_rx: Receiver<Vec<u8>>, output_tx: Sender<Vec<u8>>) {
        let (input_tx, input_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) =
            channel();
        let (output_tx, output_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) =
            channel();

        // Spawn the run_executable function in a separate thread
        let mod_host_th: thread::JoinHandle<()> = thread::spawn(move || {
            run_executable(
                "/home/puppy/mod-host/mod-host",
                &vec!["-i", "-n"],
                input_rx,
                output_tx,
            );
        });

        let result = ModHostController {
            mod_host_th,
            simulators,
            input_tx,
            output_rx,
            sent_commands: HashSet::new(),
            mh_command_queue: VecDeque::new(),
            resp_command: None,
            connections: HashMap::new(),
        };
        {
            // Ensure mod-host is going.  This is taking a gamble.  The
            // gamble is that we will get the whole response all at once.
            let resp = result.get_resp()?;
            // const MOD_HOST: &str = "mod-host> ";
            const MOD_HOST: &str = "mod-host>";
            let resp = resp.as_str().trim();
            if resp != MOD_HOST {
                panic!("Unknown response: '{resp}'.  Not: '{MOD_HOST}'");
            }
            eprintln!("Channel working: {resp}");
        }
        Ok(result)
    }

    /// Get the default value of a port
    pub fn get_default(&self, lv2_url: &str, port_symb: &str) -> String {
        let cpp = match self
            .simulators
            .iter()
            .find(|s| s.url.as_str() == lv2_url)
            .expect("Get LV2 for default value")
            .ports
            .iter()
            .find(|p| p.symbol.as_str() == port_symb)
            .expect("Get port by symbol for default value")
            .types
            .iter()
            .find(|t| matches!(t, PortType::Control(_)))
            .expect("Cannot get control port to get default")
        {
            PortType::Control(cpp) => cpp,
            _ => panic!("Control port not a control port"),
        };

        match cpp {
            ControlPortProperties::Continuous(c) => format!("{}", c.default),
            ControlPortProperties::Scale(s) => s
                .labels_values
                .first()
                .expect("Some scale values and labels getting default")
                .0
                .clone(),
        }
    }

    /// Queue a command to send to mod-host
    pub fn send_mh_cmd(&mut self, cmd: &str) {
        self.mh_command_queue.push_back(cmd.to_string());
    }

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
    pub fn get_resp(&self) -> Result<String> {
        let resp = match self.output_rx.recv() {
            Ok(t) => t,
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    err.to_string(),
                ))
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
    pub fn try_get_resp(&self) -> Result<Option<String>> {
        match self.output_rx.try_recv() {
            Ok(resp) => {
                // Got some data
                let resp: Vec<u8> = rem_trail_0(resp);
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
    pub fn get_lv2_by_url(&self, url: &str) -> Option<&Lv2> {
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

    /// Called from the event loop to send a message to mod-host
    pub fn pump_mh_queue(&mut self) {
        // self.reduce_queue();
        if !self.mh_command_queue.is_empty()
            &&
        // Only push a command if there is none or one command in flight
            self.sent_commands.is_empty()
        {
            // Safe because queue is not empty
            let cmd = self.mh_command_queue.pop_front().unwrap();
            if cmd.starts_with("connect") || cmd.starts_with("disconnect") {
                let (sp, _) = cmd
                    .char_indices()
                    .find(|x| x.1 == ' ')
                    .expect("No space in (dis)connect command");
                let connection = &cmd[sp + 1..];
                match (self.connections.get(connection), &cmd[0..sp]) {
                    (Some(ConDisconFlight::Connected), "connect") => {
                        return;
                    }
                    (Some(ConDisconFlight::Disconnected), "connect") => {}
                    (Some(ConDisconFlight::InFlight), _b) => {
                        return;
                    }
                    (Some(ConDisconFlight::Connected), "disconnect") => {}
                    (Some(ConDisconFlight::Disconnected), "disconnect") => {
                        return;
                    }
                    (None, _b) => {}
                    (a, b) => panic!("ERR Reality discontinuity: {a:?} {b:?}"),
                }
                self.connections
                    .insert(connection.to_string(), ConDisconFlight::InFlight);
            }
            self.sent_commands.insert(cmd.trim().to_string());
            eprintln!("DBG CMD SEND {cmd}");
            self.input_tx
                .send(cmd.as_bytes().to_vec())
                .expect("Send to mod-host");
        }
    }

    pub fn save_current_lv2(&self, _save_name: &str) -> bool {
        false
    }

    pub fn get_queued_count(&self) -> usize {
        self.mh_command_queue.len()
    }
}

/// Helper functions below here
/// For strings starting like `"1.0"^^<htt...` And the first quoted part
/// `1.0` is wanted.  Panic if invalid string passed
fn remove_quotes(inp: &str) -> &str {
    // The first character is a quote.  Thence the string intil the
    // next quote
    let i = inp[1..] // Exclude the first quote
	.find('"') // Find the next
	.expect("Removing quotes from {inp}.  i:{i}") // Unwrap it
	+ 1; // Why this?
    &inp[1..i]
}

/// Numbers for control ports are in the data often without a decimal
/// point.  This takes a LV2 object string and extracts the number.
/// Or panics.  It also returns the LV2 type
/// "0.5"^^<http://www.w3.org/2001/XMLSchema#decimal> -> (0.5_f64, "decimal")
fn control_number(object: &str) -> (f64, String) {
    let b = &object[1..];
    let c = b
        .chars()
        .position(|c| c == '"')
        .expect("Fnding quote in object");
    let n = &b[..c];
    let n = n.parse::<f64>().expect("control number parse");

    let b = &b[c + 1..];
    let c = b
        .chars()
        .position(|c| c == '#')
        .expect("Fnding quote in object");
    let schema = b[c + 1..b.len() - 1].to_string();
    (n, schema)
}

fn get_number_from_object(object: &str) -> &str {
    // "29"^^<http://www.w3.org/2001/XMLSchema#integer> not usize
    let c = object[1..]
        .chars()
        .position(|c| c == '^')
        .expect("Two quotes in number string");
    &object[1..c]
}

fn _port_name(data: &[&Lv2Datum]) -> String {
    data.iter()
        .filter(|&l| l.predicate == "<http://lv2plug.in/ns/lv2core#name>")
        .collect::<Vec<&&Lv2Datum>>()
        .iter()
        .fold(String::new(), |a, b| {
            // println!("Name: '{a}' + '{}'", b.object);
            a + remove_quotes(b.object.as_str())
        })
}

#[derive(Clone)]
pub struct ScaleDescription {
    pub labels: Vec<String>,
    pub values: Vec<String>,
}

/// For control ports get the important data
fn get_mmdls(
    l: &[&Lv2Datum],
    lv2_data: &[Lv2Datum],
) -> Option<(
    f64,
    f64,
    f64,
    bool,
    Option<ScaleDescription>,
    ContinuousType,
)> {
    let min_set: Vec<&Lv2Datum> =
        predicate_filter(l.iter(), "<http://lv2plug.in/ns/lv2core#minimum>")
            .into_iter()
            .copied()
            .collect();

    if min_set.is_empty() {
        eprintln!("DBG {l:?}");
        return None;
    }
    let min_set = control_number(&min_set[0].object);
    let min_type = min_set.1;
    let min = min_set.0;
    let max_set: Vec<&Lv2Datum> =
        predicate_filter(l.iter(), "<http://lv2plug.in/ns/lv2core#maximum>")
            .into_iter()
            .copied()
            .collect();
    assert!(max_set.len() == 1);
    let max_set = control_number(&max_set[0].object);
    let max_type = max_set.1;
    let max = max_set.0;
    // let max: f64 = l
    // 	 .iter()
    //   .filter(|&l| l.predicate == "<http://lv2plug.in/ns/lv2core#maximum>")
    // .collect::<Vec<&&Lv2Datum>>()
    // .iter()
    //   .fold(0.0, |a, b| a
    // 		  + control_number(b.object.as_str()));

    let default_set: Vec<&Lv2Datum> =
        predicate_filter(l.iter(), "<http://lv2plug.in/ns/lv2core#default>")
            .into_iter()
            .copied()
            .collect();
    let default_set = if default_set.len() == 1 {
        control_number(&default_set[0].object)
    } else if default_set.is_empty() {
        // What to do if not default?
        (min, min_type.clone())
    } else {
        eprintln!("Too many defaults");
        return None;
    };
    let def_type = default_set.1;
    let default = default_set.0;
    // let default: f64 =
    //      predicate_filter(l.iter(),
    // 							 "<http://lv2plug.in/ns/lv2core#default>")
    //       .iter()
    //       .fold(0.0, |a, b| a + control_number(b.object.as_str()));

    let logarithmic: bool = !predicate_filter(
        l.iter(),
        "<http://lv2plug.in/ns/lv2core#portProperty>",
    )
    .iter()
    .filter(|lv| {
        lv.object == "<http://lv2plug.in/ns/ext/port-props#logarithmic>"
    })
    .collect::<Vec<&&&Lv2Datum>>()
    .is_empty();

    // Get `scalePoints' to construct a `scale` value
    // for the Port, if it is of that type .  It will
    // have a scale if it is a Control Port with
    // discrete values.  E.g. "on"/"off", or
    // "sin"/"triangle"/"square"
    let scale_points = l
        .iter()
        .filter(|l| l.predicate == "<http://lv2plug.in/ns/lv2core#scalePoint>")
        .collect::<Vec<&&Lv2Datum>>()
        .iter()
        .map(|&l| l.object.as_str())
        .collect::<Vec<&str>>();
    let scale: Option<ScaleDescription> = if scale_points.is_empty() {
        None
    } else {
        let mut scale_description = ScaleDescription {
            labels: vec![],
            values: vec![],
        };
        for sp in scale_points.iter() {
            // For each scale point need to rescan all the
            // tripples.  Frustratingly inefficient
            let both_points: Vec<&Lv2Datum> =
                lv2_data.iter().filter(|&x| &x.subject == sp).collect();
            if both_points.len() != 2 {
                eprintln!("scale is wrong.   {both_points:?}");
                continue;
            }
            for sp in both_points {
                if sp.predicate.as_str()
                    == "<http://www.w3.org/2000/01/rdf-schema#label>"
                {
                    let label = remove_quotes(sp.object.as_str());

                    scale_description.labels.push(label.to_string());
                }
                if sp.predicate.as_str()
                    == "<http://www.w3.org/1999/02/22-rdf-syntax-ns#value>"
                {
                    scale_description.values.push(
                        get_number_from_object(sp.object.as_str()).to_string(),
                    );
                }
            }
        }
        Some(scale_description)
    };

    // For some reason it is sometimes true that not all the minimum,
    // maximum, and default values are the same type.  Use a "majority
    // rules" algorithm.
    let con_type = if max_type == min_type || max_type == def_type {
        max_type
    } else if min_type == def_type {
        min_type
    } else {
        eprintln!("All three types are diferent: {l:?}");
        "double".to_string()
    };

    Some((
        min,
        max,
        default,
        logarithmic,
        scale,
        match con_type.as_str() {
            "integer" => ContinuousType::Integer,
            "decimal" => ContinuousType::Decimal,
            "double" => ContinuousType::Double,
            _ => {
                eprintln!("{con_type}: Unknown conntrol port type");
                return None;
            }
        },
    ))
}

// Filter Lv2Datum by predicate
fn predicate_filter<'a, T: Iterator<Item = &'a &'a Lv2Datum>>(
    i: T,
    filter: &'a str,
) -> Vec<&'a &'a Lv2Datum> {
    //Vec<&'a &'a Lv2Datum> {
    i.filter(|l| l.predicate == filter)
        .collect::<Vec<&'a &'a Lv2Datum>>()
}
