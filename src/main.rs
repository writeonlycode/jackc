use clap::Parser;
use jackc::Config;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = jackc::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
