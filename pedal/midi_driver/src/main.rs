use regex::Regex;
use std::env::args;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use subprocess::Exec;
// use subprocess::Communicator;
// use subprocess::Popen;
// use subprocess::PopenConfig;
const ASEQDUMP: &str = "/usr/bin/aseqdump";
fn main() -> Result<(), Box<dyn Error>> {
    let port: String = args().nth(1).unwrap();
    run(port).unwrap();
    Ok(())
}

fn run(name: String) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let x = Exec::cmd(ASEQDUMP)
        .arg("-p")
        .arg(name.as_str())
        .stream_stdout()
        .unwrap();
    let br = BufReader::new(x);
    let re = Regex::new(r"Program change\s+\d+, program (\d+)$").unwrap();
    for (i, line) in br.lines().enumerate() {
        //32:0   Program change Program change          0, program 16
        let line = match line {
            Ok(l) => l,
            Err(err) => panic!["Died {err}"],
        };
        let cap = match re.captures(line.as_str()) {
            Some(c) => c,
            None => {
                println!("Did not match: {line}");
                continue;
            }
        };
        let sig = cap[1].parse::<i32>().unwrap();
        println!("{}: {}", i, sig);
    }
    // Command::new(aseqdump)
    //     .arg(name.as_str())
    //     .stdout(Stdio::piped())
    //     .output()
    //     .expect("Failed to execute command");
    // let z: String = output.stdout.unwrap();
    Ok(())
}
