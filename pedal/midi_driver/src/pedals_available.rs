//! Maintains the list of pedals available and the jack pipes required
//! to extablish them

//! The pedals are found using the environment variable
//! `Home120Proof`.  That points to a directory under which there is
//! `pedal/PEDALS` directory.

//! There are four pedals to match the four buttons on the MIDI foot
//! switch I am using.  One day this will be parameterised.  The
//! pedals are identified by links 'A'..'D' that point to files with
//! one jack pipe per line: `<source> <destination>`
use std::collections::hash_map::HashMap;
use std::env::vars;
use std::fs::read_dir;
use std::fs::ReadDir;
use std::path::Path;
struct PedalsAvailable {
    table: HashMap<String, Vec<(String, String)>>,
}

fn get_files_to_read() -> Vec<String> {
    let home_dir: String = match vars().find(|x| x.0 == "Home120Proof") {
        Some(s) => s.1,
        None => panic!("Home120Proof not in environment"),
    };
    let pedal_dir = format!("{home_dir}/pedal/PEDALS");
    let dir_contents: ReadDir = read_dir(pedal_dir).expect("Failed to read {pedal_dir}");
    let result: Vec<String> = dir_contents
        .filter(|x| {
            x.as_ref().unwrap().file_name().len() == 1 && x.as_ref().unwrap().file_name() != "."
        })
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect();
    Vec::new()
}

impl PedalsAvailable {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}
