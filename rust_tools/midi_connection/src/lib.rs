use std::error::Error;
pub struct MIDICommunicator<T: 'static> {
    _in_conn: Option<midir::MidiInputConnection<T>>,
    out_conn: Option<midir::MidiOutputConnection>,
}
impl<T: std::fmt::Debug + Send> MIDICommunicator<T> {
    pub fn new<F>(
        other_name: &str,
        this_name: &str,
        callback: F,
        data: T,
    ) -> Result<MIDICommunicator<T>, Box<dyn Error>>
    where
        F: FnMut(u64, &[u8], &mut T) + Send + 'static,
    {
        match Self::get_midi_connections(other_name, this_name, callback, data) {
            Ok((o_conn_in, o_conn_out)) => Ok(MIDICommunicator {
                out_conn: o_conn_out,
                _in_conn: o_conn_in,
            }),
            Err(err) => Err(err.into()),
        }
    }
    pub fn send(&mut self, msg: &[u8]) -> Result<(), Box<dyn Error>> {
        self.out_conn
            .as_mut()
            .unwrap()
            .send(&msg)
            .unwrap_or_else(|_| println!("Error when forwarding message ..."));
        Ok(())
    }
    /// Given the name of a device return an input and output connection
    /// to it
    fn get_midi_connections<F>(
        other_name: &str,
        this_name: &str,
        callback: F,
        data: T,
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
        let mut result_in: Option<midir::MidiInputConnection<T>> = None;
        let mut result_out: Option<midir::MidiOutputConnection> = None;
        // let copy_name = name.to_string();
        let source_port = other_name.to_string().into_bytes();
        let midi_out = midir::MidiOutput::new(this_name)?;
        let mut midi_in = midir::MidiInput::new(this_name)?;
        midi_in.ignore(midir::Ignore::None);

        for (index, port) in midi_out.ports().iter().enumerate() {
            // Each available output port.
            match midi_out.port_name(port) {
                Err(_) => continue,
                Ok(port_name) => {
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
                        result_out =
                            match midi_out.connect(&port, format!("{}-out", this_name).as_str()) {
                                Ok(s) => Some(s),
                                Err(err) => {
                                    eprintln!("{:?}", err);
                                    None
                                }
                            };
                        break;
                    }
                }
            }
        }

        for (i, p) in midi_in.ports().iter().enumerate() {
            let port_name = midi_in.port_name(p).unwrap().into_bytes();
            let mut accept: bool = true;
            for i in 0..port_name.len() {
                if i < source_port.len() && source_port[i] != port_name[i] {
                    accept = false;
                    break;
                }
            }
            if accept {
                let port = midi_in
                    .ports()
                    .get(i)
                    .ok_or("Invalid port number")
                    .unwrap()
                    .clone();
                result_in = match midi_in.connect(
                    &port,
                    format!("{}-in", this_name).as_str(),
                    callback,
                    data,
                ) {
                    Ok(a) => Some(a),
                    Err(err) => {
                        eprintln!("{:?}", err);
                        None
                    }
                };
                break;
            }
        }

        Ok((result_in, result_out))
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
        let port_names = MIDICommunicator::get_midi_inputs().unwrap();
        let midiConnections = MIDICommunicator::new(
            port_names.first().as_str(),
            "120-Proof-Test",
            move |_, _, _| (),
            (),
        )
        .unwrap();
    }
}
