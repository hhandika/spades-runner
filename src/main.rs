// Heru Handika
// March 2021
// MIT

mod cli;
mod finder;
mod io;
mod runner;
mod utils;


use std::time::Instant;

use clap::crate_version;

fn main() {
    let version = crate_version!();
    let time = Instant::now();
    cli::get_cli(&version);
    let duration = time.elapsed();

    println!("Execution time: {:?}", duration);
    println!("Thank you for using spades-runner v{} 😊", &version);
}

