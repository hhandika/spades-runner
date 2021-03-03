// use std::path::PathBuf;

#[allow(dead_code)]
use clap::{App, AppSettings, Arg, ArgMatches};

use crate::runner;
use crate::io;

#[allow(dead_code)]
pub fn get_cli(version: &str) {
    let args = App::new("renamer")
        .version(version)
        .about("Batch sequence assembly using SPAdes")
        .author("Heru Handika <hhandi1@lsu.edu>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("check")
                .about("Checks if fastp is installed")
            )
        
        .subcommand(
            App::new("auto")
                .about("Auto find clean reads and assembly them")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .help("Inputs a directory for auto search")
                        .takes_value(true)
                        .value_name("CLEAN-READ DIR")
                        .required(true)
                )

                .arg(
                    Arg::with_name("specify")
                        .short("s")
                        .long("specify")
                        .help("Specifies clean read directory names")
                        .takes_value(true)
                        .default_value("trimmed")
                        .value_name("DIR NAME")
                )

                .arg(
                    Arg::with_name("dry-run")
                        .long("dry")
                        .help("Checks if the program can find the correct files")
                        .takes_value(false)
                )
            )

        .subcommand(
            App::new("assembly")
                .about("Runs SPAdes")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("Inputs a config file")
                        .takes_value(true)
                        .value_name("INPUT")
                )
                
                .arg(
                    Arg::with_name("dry-run")
                        .long("dry")
                        .help("Checks if the program detect the correct files")
                        .takes_value(false)
                )

        )
        
        .get_matches();

    match args.subcommand() {
        ("auto", Some(assembly_matches)) => run_spades_auto(assembly_matches, version),
        ("assembly", Some(_)) => runner::check_spades(),
        ("check", Some(_)) => runner::check_spades(),
        _ => (),
    };
}

fn run_spades_auto(matches: &ArgMatches, version: &str) {
    let path = matches.value_of("dir").unwrap();
    let dirname = matches.value_of("specify").unwrap();

    println!("Starting spade-runner v{}\n", version);
    
    if matches.is_present("dry-run") {
        io::dry_run(path, &dirname)
    } else {
        io::auto_find_reads(path, &dirname);
    }
}