use std::io::{Read, Write};
use std::process::{ChildStderr, ChildStdout, Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, time};

/// Remove the zero bytes from the end of a`resp`
pub fn rem_trail_0(resp: Vec<u8>) -> Vec<u8> {
    let mut i = resp.as_slice().iter();
    let n = i.position(|&x| x == 0); // || x == 13 || x == 10); //.unwrap_or(resp.len());
    let n = n.unwrap_or(resp.len());
    resp[..n].to_vec()
}

/// Cannot do a non-blocking read on the ChildStdout.  Can do a
/// non-blocking read on the Receiver end of `output_tx`
fn read_child_out(mut child_stdout: ChildStdout, output_tx: Sender<Vec<u8>>) {
    thread::spawn(move || loop {
        let mut output_data = [0; 1024];
        match child_stdout.read(&mut output_data) {
            Ok(n) => {
                if n > 0 {
                    output_tx.send(output_data.to_vec()).unwrap()
                }
            }
            Err(err) => panic!("{err}: Failed reading ChildStdout"),
        };
    });
}
/// Cannot do a non-blocking read on the ChildStderr.  Can do a
/// non-blocking read on the Receiver end of `errput_tx`
fn read_child_err(mut child_stderr: ChildStderr, errput_tx: Sender<Vec<u8>>) {
    thread::spawn(move || loop {
        let mut errput_data = [0; 1024];
        match child_stderr.read(&mut errput_data) {
            Ok(n) => {
                if n > 0 {
                    errput_tx.send(errput_data.to_vec()).unwrap()
                }
            }
            Err(err) => panic!("{err}: Failed reading ChildStderr"),
        };
    });
}

/// Run the executable in `path`, aith the arguments in `args`.  It
/// will read from `input_rx` and write to `output_tx.
pub fn run_executable(
    path: &str,
    args: &Vec<&str>,
    input_rx: Receiver<Vec<u8>>,
    output_tx: Sender<Vec<u8>>,
) {
    let mut command = Command::new(path);
    for arg in args {
        command.arg(arg);
    }
    let mut child = command
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let mut stdin = child.stdin.take().unwrap();

    let target_fps = 100;
    let target_frame_time = time::Duration::from_secs(1) / target_fps;

    let (stdout_tx, stdout_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) =
        channel();
    read_child_out(stdout, stdout_tx);

    let (stderr_tx, stderr_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) =
        channel();
    read_child_err(stderr, stderr_tx);

    loop {
        // Note the time at the top of the loop, and sleep at the
        // bottom to keep the looping speed constant
        let start_time = time::Instant::now();

        // Check that the child (mod-host) process is running still
        match child.try_wait() {
            Ok(Some(s)) => {
                eprintln!("Child exited status: {s}");
                break;
            }
            Ok(None) => {
                // Child still running
            }
            Err(err) => panic!("{err}: Cannot get child status"),
        };

        // Non-blocking read from input channel, the user interface.
        // These are commands for mod-host
        if let Ok(data) = input_rx.try_recv() {
            // Strip off the zeros from the end of the input
            let mut data = rem_trail_0(data);

            // Append a new line as mod-host input is line orientated
            data.append(&mut "\n".to_string().as_bytes().to_vec());
            if !data.is_empty() {
                // `stdin` is the STDIN of the child
                stdin.write_all(&data).unwrap();
            }
        }

        // Non-blocking read from the child.
        if let Ok(s) = stdout_rx.try_recv() {
            // Non-blocking send to output channel
            let s = rem_trail_0(s); // Strip zeros

            // Send the output from mod-host to the UI
            output_tx.send(s).unwrap();
        }
        if let Ok(s) = stderr_rx.try_recv() {
            // Non-blocking send to errput channel
            let s = rem_trail_0(s); // Strip zeros
            let s = String::from_utf8(s).unwrap();
            eprint!("DBG mod-host STDERR: {s}");
        }

        // enforce duration
        let elapsed_time = start_time.elapsed();
        if elapsed_time < target_frame_time {
            thread::sleep(target_frame_time - elapsed_time);
        } else {
            eprintln!(
                "Slow in run_executable loop: {}/{}",
                elapsed_time.as_micros(),
                target_frame_time.as_micros()
            );
        }
    }

    // Kill the process
    child.kill().unwrap();
}
