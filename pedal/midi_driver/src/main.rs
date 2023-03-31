use crate::jack_connections::JackConnections;
use crate::midi::Midi;
use crate::pedals_available::get_pipes_from_file;
use crate::pedals_available::PedalsAvailable;
//use jack;
use regex::Regex;
use std::env::args;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use subprocess::Exec;
use subprocess::Redirection;
mod jack_connections;
mod midi;
mod pedals_available;

// use subprocess::Communicator;
// use subprocess::Popen;
// use subprocess::PopenConfig;
const ASEQDUMP: &str = "/usr/bin/aseqdump";
fn main() -> Result<(), Box<dyn Error>> {
    let card: String = args().nth(1).unwrap();
    run(card)
}

fn run(name: String) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let midi = Midi::new(name)?;
    let re = Regex::new(r"Program change\s+\d+, program (\d+)$").unwrap();
    let mut jack_connetions = JackConnections::new("client_name");
    let _pedals_available = PedalsAvailable::new();

    // Cache connections that are made so they can be deleted after
    // other connections made
    let mut connection_cache: Option<Vec<(String, String)>> = None;

    loop {
        // let _x = match Exec::cmd(ASEQDUMP)
        //     .arg("-p")
        //     .arg(midi.name.as_str())
        //     .stderr(Redirection::Merge)
        //     .stream_stdout()
        // {
        //     Ok(x) => {
        //         println!("Started {ASEQDUMP} -p {}", midi.name);
        //         let br = BufReader::new(x);
        //         for line in br.lines() {
        //             //32:0   Program change Program change          0, program 16
        //             let line = match line {
        //                 Ok(l) => l,
        //                 Err(err) => panic!["Died {err}"],
        //             };
        //             println!("Line: {line}");
        //             let cap = match re.captures(line.as_str()) {
        //                 Some(c) => c,
        //                 None => {
        //                     eprintln!("Did not match: {line} ");
        //                     continue;
        //                 }
        //             };
        //             let sig = cap[1].parse::<u32>().unwrap();
        //             let selected_pedal: String =
        //                 format!("{}", char::from_u32(('A' as u32) + sig).unwrap());
        //             let pipes: Vec<(String, String)> =
        //                 get_pipes_from_file(selected_pedal.as_str())?;
        //             for pipe in pipes.iter() {
        //                 match jack_connetions.make_connection(pipe.0.clone(), pipe.1.clone()) {
        //                     Ok(_) => println!("Connected"),
        //                     Err(err) => println!("Failed: {err}"),
        //                 };
        //             }
        //             if let Some(cache) = connection_cache {
        //                 for pipe in cache {
        //                     let src = pipe.0;
        //                     let dst = pipe.1;
        //                     match jack_connetions.unmake_connection(src.clone(), dst.clone()) {
        //                         Ok(_) => println!("Disconnected {src} {dst}"),
        //                         Err(err) => println!("Failed: {err} Disconnect  {src} {dst}"),
        //                     };
        //                 }
        //             }
        //             connection_cache = Some(pipes);
        //             println!("Pedal: {selected_pedal}");
        //             continue;
        //         }
        //     }
        //     Err(err) => {
        //         eprintln!("Failed: {err}");
        //     }
        // };
        sleep(Duration::from_millis(1765));
        println!("Bottom of loop");
    }
    // Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_device_names() {
        let list = Midi::list_device_names();
        assert!(list.len() > 0);
        for s in list.iter() {
            println!("{s}");
        }
    }

    #[test]
    fn test_file_names() {
        let res: Vec<String> = pedals_available::get_files_to_read();
        assert!(res.len() > 0);
    }
    #[test]
    fn test_get_pipes_from_file() {
        for p in pedals_available::get_files_to_read() {
            let pipe_pairs: Vec<(String, String)> =
                match pedals_available::get_pipes_from_file(p.as_str()) {
                    Ok(p) => p,
                    Err(err) => panic!("{}", err),
                };
            assert!(pipe_pairs.len() > 0);
        }
    }
}
