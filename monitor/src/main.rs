/// Monitor the output of all the programmes started using
/// One20Proof::run_daemon.
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
mod default_filter;
mod file_filter;
mod file_record;
mod yoshimi_out_filter;

use default_filter::DefaultFilter;
use file_record::FileRecord;
use yoshimi_out_filter::YoshimiOutFilter;

const OUTPUT_DIR: &str = "output";

/// Get the names of all the files in the output directory
fn get_file_names(output_dir_path: &Path) -> Vec<String> {
    fs::read_dir(output_dir_path)
        .unwrap()
        .map(|x| {
            x.unwrap()
                .path()
                .into_os_string()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

/// Check to see if a file has any fresh data:
/// `file_name` is complete path to the file
/// `file_position` is the position in the file to read from
/// Return the data and the new position in the file
/// The data can be an empty string and the new position the old position.
/// If the file is new but empty the data will be an empty string and
/// the new position zero
fn refresh_file(file_name: String, file_position: u64) -> io::Result<(String, u64)> {
    let mut f = fs::File::open(file_name)?;
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

    if n > 0 {
        // Got some data
        Ok((String::from_utf8(buffer).unwrap(), fsize))
    } else {
        Ok(("".to_string(), fsize))
    }
}

fn main() -> io::Result<()> {
    // Main cache of data about the files being monitored
    let mut file_store: HashMap<String, FileRecord> = HashMap::new();
    let home = env::var("Home120Proof").unwrap();
    let output_dir_path = Path::new(&home).join(OUTPUT_DIR);

    // Filters to use for the different files
    let mut default_filter = DefaultFilter {};
    let mut yoshimi_out_filter = YoshimiOutFilter::new();
    loop {
        let files = get_file_names(&output_dir_path);

        for file_name in files.iter() {
            if !file_store.contains_key(file_name) {
                // First time this file has been seen. Initialise a
                // data record for it.
                file_store.insert(file_name.clone(), FileRecord::new());
            }

            match refresh_file(
                file_name.to_string(),
                file_store.get(file_name).unwrap().position,
            ) {
                Ok((new_data, n)) => {
                    // File still exists.
                    file_store.entry(file_name.clone()).and_modify(|fr| {
                        fr.cache = new_data;
                        fr.position = n;
                    });
                }
                Err(err) => {
                    // File cannot be read.  If it exists, panic.  Else
                    // delete from cache and continue
                    match err.kind() {
                        ErrorKind::NotFound => {
                            println!("Removing filename: {}", &file_name);
                            file_store.remove(file_name);
                            continue;
                        }
                        _ => panic!("{:?}", err.kind()),
                    };
                }
            };
        }

        for (f, mut v) in &mut file_store {
            let summary = match f.as_str() {
                "yohimi.out" => v.summarise(&mut yoshimi_out_filter),
                _ => v.summarise(&mut default_filter),
            };
            for s in summary.iter() {
                println!("f: {}: {}", f, s.len());
            }
            v.cache = String::new();
        }
        thread::sleep(time::Duration::from_millis(100));
    }
    // Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    /// A bit of a convoluted test.  Cannot think of a simpler way to test
    fn test_file_names() {
        use std::env::temp_dir;
        use std::fs::create_dir;
        use std::path::Path;
        use std::path::PathBuf;
        use std::process;
        let temp_dir_name = format!("test_dir_test_file_names{}", process::id());
        let mut test_path: PathBuf = temp_dir();
        test_path.push(temp_dir_name.as_str());

        let test_dir_path = test_path.as_path();
        assert!(!Path::exists(test_dir_path));
        create_dir(test_dir_path).unwrap();
        assert!(Path::exists(test_dir_path));
        assert!(Path::is_dir(test_dir_path));

        let file_names = get_file_names(test_dir_path);
        assert!(file_names.is_empty());

        let temp_file_name = format!("test_file_{}", process::id());
        let mut test_file_path = test_path.clone();
        test_file_path.push(temp_file_name.as_str());
        let test_file_path = test_file_path.as_path();
        let _file = fs::File::create(test_file_path).unwrap();
        assert!(Path::is_file(test_file_path));

        let file_names = get_file_names(test_dir_path);
        assert!(file_names.len() == 1);

        // Clean up
        fs::remove_file(test_file_path).unwrap();
        fs::remove_dir(test_dir_path).unwrap();
        assert!(!Path::exists(test_dir_path));
    }

    #[test]
    /// A convoluted test to test refreshing files

    fn test_refresh_file() {
        use std::env::temp_dir;
        use std::fs::create_dir;
        use std::path::Path;
        use std::path::PathBuf;
        use std::process;
        let temp_dir_name = format!("test_dir_test_refresh_files{}", process::id());
        let mut test_path: PathBuf = temp_dir();
        test_path.push(temp_dir_name.as_str());

        let test_dir_path = test_path.as_path();
        assert!(!Path::exists(test_dir_path));
        create_dir(test_dir_path).unwrap();
        assert!(Path::exists(test_dir_path));
        assert!(Path::is_dir(test_dir_path));

        let temp_file_name = format!("test_file_{}", process::id());
        let mut test_file_path = test_path.clone();
        test_file_path.push(temp_file_name.as_str());
        let test_file_path = test_file_path.as_path();
        let mut file = fs::File::create(test_file_path).unwrap();
        let file_name: String = test_file_path.as_os_str().to_str().unwrap().to_string();
        assert!(Path::is_file(test_file_path));
        let (contents, position): (String, u64) = refresh_file(file_name.clone(), 0).unwrap();
        assert!(contents.is_empty());
        assert!(position == 0);
        let test_string: String = "abcdefg".to_string();
        file.write_all(test_string.as_bytes()).unwrap();

        let (contents, position): (String, u64) = refresh_file(file_name.clone(), 0).unwrap();
        assert!(contents == test_string);
        assert!(position == test_string.len() as u64);

        let (contents, position): (String, u64) = refresh_file(file_name, 1).unwrap();
        assert!(contents == test_string[1..]);
        assert!(position == test_string.len() as u64);

        // Cleanup
        fs::remove_file(test_file_path).unwrap();
        fs::remove_dir(test_dir_path).unwrap();
        assert!(!Path::exists(test_dir_path));
    }
}
