//! Maintains the list of pedals available and the jack pipes required
//! to extablish them

//! The pedals are found using the environment variable
//! `Home120Proof`.  That points to a directory under which there is
//! `pedal/PEDALS` directory.

//! There are four pedals to match the four buttons on the MIDI foot
//! switch I am using.  One day this will be parameterised.  The
//! pedals are identified by links 'A'..'D' that point to files with
//! one jack pipe per line: `<source> <destination>`
use std::env::vars;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

/// Passed the name of a file that defines a pedal (use just the name,
/// path will be added here) return the Jack pipe names used to
/// initiale it
pub fn get_pipes_from_file(file_name: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    match vars().find(|x| x.0 == "Home120Proof") {
        Some(s) => {
            let file_name: String = format!("{}/pedal/PEDALS/{file_name}", s.1);
            println!("File name: {file_name}");
            let file = File::open(file_name)?;
            let buf: Lines<BufReader<File>> = BufReader::new(file).lines();
            Ok(buf
                .filter(|line| line.as_ref().map(|s| !s.is_empty()).unwrap_or(false))
                .map(|x| {
                    let x = x.unwrap();
                    let s_x: Vec<&str> = x.split(' ').collect();
                    println!("x {x} s_x {s_x:?}");
                    assert!(s_x.len() == 2);
                    (String::from(s_x[0]), String::from(s_x[1]))
                })
                .collect())
        }
        None => panic!("Home120Proof not in environment"),
    }
}

// impl PedalsAvailable {
//     pub fn new() -> Self {
//         Self {
//             // table: HashMap::new(),
//         }
//     }
// }
