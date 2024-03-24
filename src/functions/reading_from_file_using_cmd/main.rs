use super::{read_file, read_from_cmd};
use std::process;

pub fn main() {
    let config = read_from_cmd::Config::new().unwrap_or_else(|err| {
        println!("problem in parsing {}", err);
        process::exit(1);
    });

    if let Err(e) = read_file::readfile(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
