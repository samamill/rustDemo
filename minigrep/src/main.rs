extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
	let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Unable to parse arguments: {}", err);
        process::exit(1);
    });
	
	if let Err(err) = minigrep::run(config) {
        eprintln!("Unable to search file: {}", err);
        process::exit(1);
    };
}

