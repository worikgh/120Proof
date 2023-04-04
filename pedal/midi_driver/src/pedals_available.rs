//! Maintains the list of pedals available and the jack pipes required
//! to extablish them

//! The pedals are found using the environment variable
//! `Home120Proof`.  That points to a directory under which there is
//! `pedal/PEDALS` directory.

//! There are four pedals to match the four buttons on the MIDI foot
//! switch I am using.  One day this will be parameterised.  The
//! pedals are identified by links 'A'..'D' that point to files with
//! one jack pipe per line: `<source> <destination>`
// use std::collections::hash_map::HashMap;
use std::env::vars;
// use std::fs::ReadDir;
// use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
// pub struct PedalsAvailable {
//     //table: HashMap<String, Vec<(String, String)>>,
// }

// pub fn get_files_to_read() -> Vec<String> {
//     let home_dir: String = match vars().find(|x| x.0 == "Home120Proof") {
//         Some(s) => s.1,
//         None => panic!("Home120Proof not in environment"),
//     };
//     let pedal_dir = format!("{home_dir}/pedal/PEDALS");
//     read_dir(pedal_dir)
//         .expect("Failed to read {pedal_dir}")
//         .filter(|x| {
//             x.as_ref().unwrap().file_name().len() == 1 && x.as_ref().unwrap().file_name() != "."
//         })
//         .map(|x| x.unwrap().file_name().into_string().unwrap())
//         .collect()
// }

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
                .map(|x| {
                    let x = x.unwrap();
                    let s_x: Vec<&str> = x.split(' ').collect();
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
