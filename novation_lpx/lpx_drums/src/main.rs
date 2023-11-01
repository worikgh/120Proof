use crate::section::Section;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;
mod lpx_drum_error;
mod section;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Pass name of drum definition, JSON formatted, file as sole argument");
    }
    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let result: Vec<Section> = Section::parse_json(&content)?;
    if let Err(err) = Section::check_sections(&result) {
        panic!("{err}");
    }
    Ok(())
}
