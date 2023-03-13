use regex::Regex;
use std::env::args;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use subprocess::Exec;
use subprocess::Redirection;
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
    let re = Regex::new(r"Program change\s+\d+, program (\d+)$").unwrap();
    loop {
        let _x = match Exec::cmd(ASEQDUMP)
            .arg("-p")
            .arg(name.as_str())
            .stderr(Redirection::Merge)
            .stream_stdout()
        {
            Ok(x) => {
                println!("Started {ASEQDUMP} -p {name}");
                let br = BufReader::new(x);
                for line in br.lines() {
                    //32:0   Program change Program change          0, program 16
                    let line = match line {
                        Ok(l) => {
                            // println!(">{l}");
                            l
                        }
                        Err(err) => panic!["Died {err}"],
                    };
                    let cap = match re.captures(line.as_str()) {
                        Some(c) => c,
                        None => {
                            println!("Did not match: {line} ");
                            continue;
                        }
                    };
                    let sig = cap[1].parse::<i32>().unwrap();
                    println!("Pedal: {sig}");
                    continue;
                }
            }
            Err(err) => {
                eprintln!("Failed: {err}");
            }
        };
        sleep(Duration::from_millis(765));
        println!("Bottom of loop");
    }
    // Ok(())
}
