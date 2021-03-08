// Heru Handika
// March 2021
// MIT

mod cleaner;
mod cli;
mod finder;
mod io;
mod parser;
mod runner;
mod utils;


use std::time::Instant;

use clap::crate_version;

fn main() {
    let version = crate_version!();
    let time = Instant::now();
    cli::get_cli(&version);
    let duration = time.elapsed();

    if duration.as_secs() < 60 {
        println!("Execution time: {:?}", duration);
    } else {
        utils::parse_duration(duration.as_secs());
    }

    println!("Thank you for using spades-runner v{} 😊", &version);
}

