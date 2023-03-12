use midir::MidiIO;
use midir::MidiInput;
use midir::MidiOutput;
use midir::MidiOutputPort;
use std::env::args;
use std::error::Error;
use std::io::stdin;
use std::iter::zip;

fn main() {
    let port_name_in: String = args().nth(1).unwrap();
    let port_name_out: String = match args().nth(1) {
        Some(p) => p,
        None => port_name_in.clone(),
    };
    run(port_name_in, port_name_out).unwrap();
}

fn run(name_of_port_in: String, name_of_port_out: String) -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("120Proof-midi-pedal-driver-out")?;
    let midi_out_ports = midi_out.ports();
    for (i, o) in midi_out_ports.iter().enumerate() {
        println!("{} {}", i, midi_out.port_name(o)?);
    }

    let _output = get_output_port(name_of_port_out.as_str()).unwrap();
    handle_input(name_of_port_in.as_str()).unwrap();

    let mut input = String::new();
    println!("Running....");
    stdin().read_line(&mut input)?; // wait for next enter key press
    println!("Closing connection");
    Ok(())
}

fn find_port<T: MidiIO>(name: &str, midi_io: &T) -> Result<T::Port, Box<dyn Error>> {
    let ports = midi_io.ports();
    let port_names: Vec<String> = ports
        .iter()
        .map(|p| midi_io.port_name(p).unwrap())
        .collect();
    let mut names_ports = zip(ports, port_names);
    let port_name = names_ports
        .find(|x| &x.1 == name)
        .expect(format!("{} not found", name).as_str());
    Ok(port_name.0)
}

fn handle_input(port_name: &str) -> Result<(), Box<dyn Error>> {
    let midi_input = MidiInput::new("120Proof-midi-pedal-driver")?;
    let input_port = match find_port(port_name, &midi_input) {
        Ok(p) => p,
        Err(err) => panic!("Failed to bet port {port_name}: {err}"),
    };
    let _ = midi_input.connect(
        &input_port,
        "midir-read-input",
        move |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
        },
        (),
    )?;
    Ok(())
}

fn get_output_port(port_name: &str) -> Result<MidiOutputPort, Box<dyn Error>> {
    let midi_output = MidiOutput::new("120Proof-midi-pedal-driver")?;
    find_port(port_name, &midi_output)
}
