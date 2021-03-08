use std::path::PathBuf;
use clap::{App, AppSettings, Arg, ArgMatches};

use crate::cleaner;
use crate::runner;
use crate::io;


pub fn get_cli(version: &str) {
    let args = App::new("SPAdes-runner")
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

                .arg(
                    Arg::with_name("threads")
                        .short("t")
                        .long("threads")
                        .help("Sets number of threads")
                        .takes_value(true)
                        .value_name("THREAD-NUM")
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

                .arg(
                    Arg::with_name("threads")
                        .short("t")
                        .long("threads")
                        .help("Sets number of threads")
                        .takes_value(true)
                        .value_name("THREAD-NUM")
                )

        )

        .subcommand(
            App::new("clean")
                .about("Clean unused SPAdes files.")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .help("Inputs a directory for cleaning")
                        .takes_value(true)
                        .value_name("DIR")
                        .required(true)
                )
            )
        
        .get_matches();

    match args.subcommand() {
        ("auto", Some(clean_matches)) => run_spades_auto(clean_matches, version),
        ("assembly", Some(assembly_matches)) => run_spades(assembly_matches, version),
        ("check", Some(_)) => runner::check_spades(),
        ("clean", Some(clean_matches)) => clean_spades_files(clean_matches),
        _ => (),
    };
}

fn run_spades_auto(matches: &ArgMatches, version: &str) {
    let path = matches.value_of("dir").unwrap();
    let dirname = matches.value_of("specify").unwrap();
    let threads = get_thread_num(matches);
    
    if matches.is_present("dry-run") {
        io::auto_dryrun(path, &dirname)
    } else {
        println!("Starting spade-runner v{}...\n", version);
        io::auto_process_input(path, &dirname, threads);
    }
}

fn run_spades(matches: &ArgMatches, version: &str) {
    let path = matches.value_of("input").unwrap();
    let threads = get_thread_num(matches);
    
    if matches.is_present("dry-run") {
        io::dryrun(path)
    } else {
        println!("Starting spade-runner v{}...\n", version);
        io::process_input(path, threads);
    }
}

fn clean_spades_files(matches: &ArgMatches) {
    let path = PathBuf::from(matches.value_of("dir").unwrap());
    cleaner::clean_spades_files(&path);
}

fn get_thread_num(matches: &ArgMatches) -> Option<usize> {
    let mut threads = None;

    if matches.is_present("threads") {
        let num = matches.value_of("threads");
        match num {
            Some(n) => threads = Some(n.parse::<usize>().unwrap()),
            None => panic!("INVALID THREAD NUMBERS!"), 
        }
    }

    threads
}