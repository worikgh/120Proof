use chrono::Local;
use chrono::Utc;
/// Monitor the output of all the programmes started using
/// One20Proof::run_daemon.
extern crate sensors;
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
mod filter_rules;
mod mod_host_out_filter;
use mod_host_out_filter::ModHostOutFilter;
mod lpx_control_err_filter;
mod pd_err_filter;
mod yoshimi_err_filter;
use default_filter::DefaultFilter;
mod yoshimi_out_filter;
use file_record::FileRecord;
use lpx_control_err_filter::LPXControlErrFilter;
use pd_err_filter::PdErrFilter;
use regex::Regex;
use yoshimi_err_filter::YoshimiErrFilter;
use yoshimi_out_filter::YoshimiOutFilter;

const OUTPUT_DIR: &str = "output";
fn get_output_dir() -> String {
    format!("{}/{}", env::var("Home120Proof").unwrap(), OUTPUT_DIR)
}

/// Given a file name, of a file with data to monitor, convert it to a
/// complete path so it can be opened
fn complete_path(file_name: &str) -> String {
    format!("{}/{}", get_output_dir(), file_name)
}

/// Get the status of mod-host instances.  Ther may be 0, 1, or 2
/// instances.  One run by user "modep"

/// Get the names of all the files in the output directory.
/// `output_dir_path` is complete path to the output directory
fn get_file_names(output_dir_path: &Path) -> Vec<String> {
    fs::read_dir(output_dir_path)
        .unwrap()
        .map(|x| {
            x.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

/// The output files from 120Proof havce form: <Name>.<PID>.err and
/// <Name>.<PID>.out  This function breaks out the name and PID
fn decode_file_name(file_name: &str) -> Option<(String, usize)> {
    let regex = Regex::new(r"^(.+)\.(\d+)\.([erout]{3}$)").unwrap();
    match regex.captures(file_name) {
        Some(caps) => {
            let text1: &str = caps.get(1).unwrap().as_str();
            let text2: &str = caps.get(2).unwrap().as_str();
            let text3: &str = caps.get(3).unwrap().as_str();
            Some((
                format!("{}.{}", text1, text3,),
                text2.parse::<usize>().unwrap(),
            ))
        }
        None => None,
    }
}

/// Check to see if a file has any fresh data: `file_name` is complete
/// path to the file `file_position` is the position in the file to
/// read from Return the data and the new position in the file The
/// data can be an empty string and the new position the old position.
/// If the file is new but empty the data will be an empty string and
/// the new position zero. (The returned u64 is always equal to the
/// file size???)
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
//use lpx_control_err_filter::test_process_texta;
fn main() -> io::Result<()> {
    // Main cache of data about the files being monitored
    // test_process_texta();
    let mut file_store: HashMap<String, FileRecord> = HashMap::new();
    let home = get_output_dir();
    let output_dir_path = Path::new(&home);

    // Filters to use for the different files
    let mut lpx_control_err_filter = LPXControlErrFilter::new();
    let mut default_filter = DefaultFilter {};
    let mut mod_host_out_filter = ModHostOutFilter::new();
    let mut y_err_filters: HashMap<usize, YoshimiErrFilter> = HashMap::new();
    let mut y_out_filters: HashMap<usize, YoshimiOutFilter> = HashMap::new();
    let mut pd_err_filters: HashMap<usize, PdErrFilter> = HashMap::new();
    let fps = 4_i64;
    loop {
	let  tol = Utc::now();
	let tol = tol.timestamp_micros();
        // let sensors = Sensors::new();
        // // cpu_thermal-virtual-0 (on Virtual device): F => temp1  SF => temp1_input = 67.679
        // // rpi_volt-isa-0000 (on ISA adapter): F => in0  SF => in0_lcrit_alarm = 0

        // let chip: Chip = sensors
        //     .into_iter()
        //     .find(|x| x.get_name().unwrap() == "cpu_thermal-virtual-0")
        //     .unwrap();
        // let feature: Feature = chip
        //     .into_iter()
        //     .find(|x| x.get_label().unwrap() == "temp1")
        //     .unwrap();
        // let sub_feature: Subfeature = feature
        //     .into_iter()
        //     .find(|x| x.name() == "temp1_input")
        //     .unwrap();

        // Get the time, and modep status every loop and report when they change
        let mut cached_timestamp: String = "".to_string();
        let  _modep_status: String = "".to_string();

        let files = get_file_names(output_dir_path);

        for file_name in files.iter() {
            if !file_store.contains_key(file_name) {
                // First time this file has been seen. Initialise a
                // data record for it.
                file_store.insert(file_name.clone(), FileRecord::new());
            }

            match refresh_file(
                complete_path(file_name),
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
                            file_store.remove(file_name);
                            continue;
                        }
                        _ => panic!("{:?}", err.kind()),
                    };
                }
            };
        }

        // Summarise the data.  Depending on the filename
        for (f,  new_data) in &mut file_store {
            let summary = match decode_file_name(f.as_str()) {
                Some((name, pid)) => new_data.summarise(
                    Some(pid),
                    match name.as_str() {
                        "yoshimi.err" => {
                            y_err_filters
                                .entry(pid)
                                .or_insert_with(|| YoshimiErrFilter::new(pid));
                            y_err_filters.get_mut(&pid).unwrap()
                        } //yoshimi_err_filter,
                        "pd.err" => {
                            pd_err_filters
                                .entry(pid)
                                .or_insert_with(|| PdErrFilter::new(pid));
                            pd_err_filters.get_mut(&pid).unwrap()
                        } //pd_err_filter,
                        "yoshimi.out" => {
                            y_out_filters
                                .entry(pid)
                                .or_insert_with(|| YoshimiOutFilter::new(pid));
                            y_out_filters.get_mut(&pid).unwrap()
                        } //yoshimi_out_filter,
                        "lpx_controll.err" => &mut lpx_control_err_filter,
                        "mod-host.out" => &mut mod_host_out_filter,
                        _ => &mut default_filter,
                    },
                ),
                None => new_data.summarise(None, &mut default_filter),
            };
            if !summary.is_empty() {
                // Maintain time
                let now: String = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                if now != cached_timestamp {
                    cached_timestamp = now;
                    println!("TS: {}", cached_timestamp,);
                }
                for s in summary.iter() {
                    println!("f: {}: {}", f, s);
                }
            }
            new_data.cache = String::new();
        }
	let bol = Utc::now().timestamp_micros();
	let diff = bol - tol;
	let target = 1_000_000 / fps;
	let sleep_time = target - diff;
	if sleep_time > 0 {
            thread::sleep(time::Duration::from_micros(sleep_time as u64));
	}else{
	    eprintln!("Overrun: {sleep_time}");
	}
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
    fn test_decode_filename() {
        let file_name: &str = "Name.9999.out";
        match decode_file_name(file_name) {
            Some((name, pid)) => {
                assert!(name == "Name.out", "name: {}", name);
                assert!(pid == 9999);
            }
            None => panic!("Failed to decode {}", file_name),
        };

        let file_name: &str = "Name.9998.err";
        match decode_file_name(file_name) {
            Some((name, pid)) => {
                assert!(name == "Name.err");
                assert!(pid == 9998);
            }
            None => panic!("Failed to decode {}", file_name),
        };

        let file_name: &str = "Name.9q998.err";
        match decode_file_name(file_name) {
            Some((name, pid)) => panic!(
                "Decoded invalid filename: {}.  name: {} pid: {}",
                file_name, name, pid
            ),

            None => assert!(true),
        };
    }
    #[test]
    /// A convoluted test to test refreshing files
    fn test_refresh_file() {
        use std::env::temp_dir;
        use std::fs::create_dir;
        use std::path::Path;
        use std::path::PathBuf;
        use std::process;

        // Make a directory to do experiments in:  First the name
        let temp_dir_name = format!("test_dir_test_refresh_files{}", process::id());
        let mut test_path_buf: PathBuf = temp_dir();
        test_path_buf.push(temp_dir_name.as_str());

        // The complete path to test directory
        let test_dir_path = test_path_buf.as_path();

        // Create the temporary directory
        assert!(!Path::exists(test_dir_path));
        create_dir(test_dir_path).unwrap();
        assert!(Path::exists(test_dir_path));
        assert!(Path::is_dir(test_dir_path));

        // Make a test file to use: First create the path to the file
        let temp_file_name = format!("test_file_{}", process::id());
        let mut test_file_path_buf = test_path_buf.clone();
        test_file_path_buf.push(temp_file_name.as_str());
        let test_file_path = test_file_path_buf.as_path();
        assert!(!Path::exists(test_file_path));

        // Create the test file
        let mut file = fs::File::create(test_file_path).unwrap();
        assert!(Path::exists(test_file_path));
        assert!(Path::is_file(test_file_path));

        // The name of the test file as a string
        let file_name: String = test_file_path.as_os_str().to_str().unwrap().to_string();

        // Test `refresh_file` on the empty file
        let (contents, position): (String, u64) = match refresh_file(file_name.clone(), 0) {
            Ok(tuple_2) => tuple_2,
            Err(err) => panic!("Failed to refresh: {}.  Err: {}", &file_name, err),
        };
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
