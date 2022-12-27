use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::SeekFrom;
use std::path::Path;
use std::thread;
use std::time;
mod file_filter;
mod file_record;
// use file_filter::FileFilter;
use file_record::FileRecord;

fn main() -> io::Result<()> {
    let mut file_store: HashMap<String, FileRecord> = HashMap::new();
    let home = env::var("Home120Proof").unwrap();
    let output_dir_path = Path::new(&home).join("output");

    loop {
        let files = fs::read_dir(output_dir_path.clone()).unwrap();
        for path in files {
            let file_name: String = format!(
                "{}/{}",
                output_dir_path.display(),
                path.unwrap().file_name().as_os_str().to_str().unwrap()
            );

            if !file_store.contains_key(&file_name) {
                file_store.insert(file_name.clone(), FileRecord::new());
            }
            let mut f = match fs::File::open(&file_name) {
                Ok(f) => f,
                Err(err) => {
                    // File cannot be read.  If it exists, panic.  Else
                    // delete from cache and continue
                    match err.kind() {
                        ErrorKind::NotFound => {
                            println!("Removing filename: {}", &file_name);
                            file_store.remove(&file_name);
                            continue;
                        }
                        _ => panic!("{:?}", err.kind()),
                    };
                }
            };
            let file_position: u64 = file_store[&file_name].position;
            let fsize = f.metadata().unwrap().len();
            if fsize < file_position {
                // File has been reset.  Read from start
                f.seek(SeekFrom::Start(0)).unwrap();
            } else {
                // File is in play read from position
                f.seek(SeekFrom::Start(file_position)).unwrap();
            }
            let mut buffer: Vec<u8> = Vec::new();
            let n = f.read_to_end(&mut buffer)?;

            // Is this needed since we did `read_to_end`?
            f.seek(SeekFrom::Current(n as i64)).unwrap();

            if n > 0 {
                // Got some data
                let new_data = String::from_utf8(buffer).unwrap();
                file_store.entry(file_name.clone()).and_modify(|fr| {
                    fr.cache += new_data.as_str();
                    fr.position += n as u64;
                });
            }
        }

        for (f, mut v) in &mut file_store {
            let summary = v.summarise(None);
            if summary.len() > 0 {
                println!("f: {}: {}", f, summary);
                v.cache = String::new();
            }
        }
        thread::sleep(time::Duration::from_millis(100));
    }
    // Ok(())
}
