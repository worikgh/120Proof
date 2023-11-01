use std::error::Error;

struct SelectPortError;
struct OutConnError;
impl std::fmt::Display for SelectPortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SelectPortError").finish()
    }
}

impl std::fmt::Debug for SelectPortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SelectPortError").finish()
    }
}
impl std::fmt::Display for OutConnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("OutConnError").finish()
    }
}

impl std::fmt::Debug for OutConnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("OutConnError").finish()
    }
}
impl Error for SelectPortError {}
impl Error for OutConnError {}

/// From midir/examples return a port
fn select_port<T: midir::MidiIO>(midi_io: &T, name: &str) -> Result<T::Port, Box<dyn Error>> {
    let midi_ports = midi_io.ports();
    let len = name.len();

    for (i, p) in midi_ports.iter().enumerate() {
        let port_name = midi_io.port_name(p)?;
        let prefix_port_name = port_name
            .as_str()
            .chars()
            // .skip(0) Unsure what this was for.  `skip(0)` does nothing
            .take(len)
            .collect::<String>();

        if prefix_port_name == name {
            // Found port
            match midi_ports.get(i) {
                Some(p) => return Ok(p.clone()),
                None => panic!("????"),
            };
        }
    }
    Err(Box::new(SelectPortError))
}

pub struct MIDICommunicator<T: 'static> {
    _in_conn: Option<midir::MidiInputConnection<T>>,
    out_conn: Option<midir::MidiOutputConnection>,
}
impl std::fmt::Debug for MIDICommunicator<()> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MIDICommunicator").finish()
    }
}
impl<T: std::fmt::Debug + Send> MIDICommunicator<T> {
    /// Create a MIDICommunicator.  `other_name` is the device that
    /// will be connected to.  `this_name` is the device that this
    /// creates that other devices connect to.  `callback` is passed
    /// to the input connection to be called with each incoming MIDI
    /// message.  `data` is passed to `callback` in the third
    /// parameter. `inout` defines whether to make an incoming,
    /// outgoing, or bidirectional connection.  If it is 1 make
    /// incoming, if it is 2 make outgoing, if it is 3 make both.  Any
    /// other value is an error
    pub fn new<F>(
        other_name: &str,
        this_name: &str,
        callback: F,
        data: T,
        inout: u8,
    ) -> Result<MIDICommunicator<T>, Box<dyn Error>>
    where
        F: FnMut(u64, &[u8], &mut T) + Send + 'static,
    {
        let connections =
            Self::get_midi_connections(other_name, this_name, Some(callback), data, inout);
        match connections {
            Ok((o_conn_in, o_conn_out)) => Ok(MIDICommunicator {
                out_conn: o_conn_out,
                _in_conn: o_conn_in,
            }),
            Err(err) => Err(err),
        }
    }
    pub fn send(&mut self, msg: &[u8]) -> Result<(), Box<dyn Error>> {
        match self.out_conn.as_mut() {
            Some(midi_out_conn) => match midi_out_conn.send(msg) {
                Ok(()) => Ok(()),
                Err(err) => Err(Box::new(err)),
            },
            None => {
                eprintln!("Error when forwarding message");
                Err(Box::new(OutConnError))
            }
        }
    }
    /// Given the name of a device return an input and output
    /// connection to it.  `other_name` is the device that will be
    /// connected to.  `this_name` is the device that this creates
    /// that other devices connect to.  `callback` is called for any
    /// data recieved.  `T` is the data passed to the callback, and
    /// `inout` is a two bit bitfield: 1 => input connection only, 2
    /// => output connection only, 3 => both
    fn get_midi_connections<F>(
        other_name: &str,
        this_name: &str,
        callback: Option<F>,
        data: T,
        inout: u8,
    ) -> Result<
        (
            Option<midir::MidiInputConnection<T>>,
            Option<midir::MidiOutputConnection>,
        ),
        Box<dyn Error>,
    >
    where
        F: FnMut(u64, &[u8], &mut T) + Send + 'static,
    {
        // The values to return
        let mut result_in: Option<midir::MidiInputConnection<T>> = None;
        let mut result_out: Option<midir::MidiOutputConnection> = None;

        // if the caller asked for it make an outgoing connection
        if inout > 1 && !other_name.is_empty() {
            // An instance of MidiOutput is required for anything
            // related to MIDI output
            let midi_out = midir::MidiOutput::new(this_name)?;

            println!("other_name: {other_name}");
            let source_port = other_name.to_string().into_bytes();

            for (index, port) in midi_out.ports().iter().enumerate() {
                // Each available output port.
                match midi_out.port_name(port) {
                    Err(_) => continue,
                    Ok(port_name) => {
                        println!("port_name: {port_name}");
                        let port_name = port_name.into_bytes();
                        let mut accept: bool = true;

                        for i in 0..port_name.len() {
                            if i < source_port.len() && source_port[i] != port_name[i] {
                                accept = false;
                                break;
                            }
                        }
                        if accept {
                            // Can build an output connection
                            let port = midi_out
                                .ports()
                                .get(index)
                                .ok_or("Invalid port number")
                                .unwrap()
                                .clone();

                            result_out = match midi_out
                                .connect(&port, format!("{}-out", this_name).as_str())
                            {
                                Ok(s) => Some(s),
                                Err(err) => {
                                    eprintln!("Could not connect {:?}", err);
                                    None
                                }
                            };
                            break;
                        }
                    }
                }
            }
        }

        // `inout` == 2 implies only create an outgoing connection.
        // So if `inout` is not 2 make incoming connection
        if inout != 2 {
            let mut midi_in = midir::MidiInput::new(this_name)?;
            midi_in.ignore(midir::Ignore::None);
            let port = select_port(&midi_in, other_name)?;
            result_in = match midi_in.connect(
                &port,
                format!("{}-in", this_name).as_str(),
                callback.unwrap(),
                data,
            ) {
                Ok(a) => Some(a),
                Err(err) => {
                    eprintln!("{:?}", err);
                    None
                }
            };
        }
        // Check the results and return
        match inout {
            1 =>
            // Input only
            {
                if result_in.is_some() {
                    Ok((result_in, None))
                } else {
                    Err("Input connection failed".into())
                }
            }
            2 =>
            // Output only
            {
                if result_out.is_some() {
                    Ok((None, result_out))
                } else {
                    let msg: String = format!(
                        "Output connection failed. inout: 2 this_name: {} other_name: {}",
                        this_name, other_name
                    );
                    Err(msg.into())
                }
            }
            3 =>
            // Both
            {
                if result_in.is_some() && result_out.is_some() {
                    Ok((result_in, result_out))
                } else {
                    match (&result_in, &result_out) {
                        (Some(_), Some(_)) => Ok((result_in, result_out)),
                        (None, Some(_)) => Err("Input connection failed.  Output Ok.".into()),
                        (Some(_), None) => Err("Input connection Ok.  Output Failed.".into()),
                        (None, None) => Err("Input connection failed.  Output Failed.".into()),
                    }
                }
            }
            _ => panic!("inout parameter is invalid: {}", inout),
        }
    }

    // Lists midi devices that can be used as inputs
    pub fn get_midi_inputs() -> Result<Vec<String>, Box<dyn Error>> {
        let midi_in = midir::MidiInput::new("120 Proof")?;
        let mut result: Vec<String> = Vec::new();
        for (_, p) in midi_in.ports().iter().enumerate() {
            result.push(midi_in.port_name(p).unwrap().clone())
        }
        Ok(result)
    }
    // Lists midi devices that can be used as outputs
    pub fn get_midi_outputs() -> Result<Vec<String>, Box<dyn Error>> {
        let midi_out = midir::MidiOutput::new("120 Proof")?;
        let mut result: Vec<String> = Vec::new();
        for (_, p) in midi_out.ports().iter().enumerate() {
            result.push(midi_out.port_name(p).unwrap().clone())
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn test_midi_connections() {
        let port_names: Vec<String> = MIDICommunicator::<()>::get_midi_inputs().unwrap();
        let _midi_connections = MIDICommunicator::new(
            port_names.first().unwrap().as_str(),
            "120-Proof-Test",
            move |_, _, _| (),
            (),
            3,
        )
        .unwrap();
    }
}
