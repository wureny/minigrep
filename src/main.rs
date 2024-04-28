use std::{env, process};
use std::ffi::CString;
use std::fs;
use std::io::Read;
use std::error::Error;
use minigrep::Config;
use minigrep::run;

fn main() {
    let args : Vec<String> = env::args().collect();
   // let (query,file_path) = parse_config(&args);
   // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}\n", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    dbg!(args);
}

