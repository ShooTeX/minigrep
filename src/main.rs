use std::process;

use clap::Parser;
use minigrep::Config;

fn main() {
    let config = Config::parse();

    println!("Searching for `{}`", config.query);
    println!("In file `{}`", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
