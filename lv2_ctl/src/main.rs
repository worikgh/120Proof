///App code taken from https://ratatui.rs/tutorials/counter-app/
use app::App;
use mod_host_controller::ModHostController;
use std::fs::File;
use std::io::BufRead;

mod app;
mod colours;
mod dialogue;
mod lv2;
mod lv2_simulator;
mod lv2_stateful_list;
mod mod_host_controller;
mod port;
mod port_table;
mod run_executable;
//mod test_data;
use std::env;
//use std::io::Cursor;
//use test_data::test_data;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let reader: Box<dyn std::io::BufRead> = //if args.len() == 1 || &args[1] != "d"
   {
      let fnm = if args.len() == 1 {
         "../../lv2.dat"
      } else {
         &args[1]
      };
      let file = if let Ok(f) = File::open(fnm) {
         f
      } else if let Ok(f) = File::open("lv2.dat") {
         f
      } else {
         panic!("Cannot find data")
      };
      Box::new(std::io::BufReader::new(file))
   } // else {
   //    let test_data = test_data();
   //    Box::new(std::io::BufReader::new(Cursor::new(test_data)))
   // }
    ;
    let mut mod_host_controller: ModHostController =
        ModHostController::get_lv2_controller(
            reader.lines(), 
        )?;
    // Start user interface.  Loop until user quits
    App::run(&mut mod_host_controller).expect("Running app");

    mod_host_controller
        .mod_host_th
        .join()
        .expect("Joining mod-host thread");

    Ok(())
}
