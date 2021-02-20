use std::process::Command;
use std::str;
use std::path::PathBuf;

fn main() {
    let r1 = PathBuf::from("data/sample_buno_r1.fastq.gz");
    let r2 = PathBuf::from("data/sample_buno_clean_r1.fastq.gz");
    check_spades();
    run_spades(&r1, &r2)

    // println!("Hello, world!");
}

fn check_spades() {
    let out = Command::new("spades")
        .arg("--version")
        .output()
        .unwrap();

    if out.status.success() {
        println!("[OK]\t{}", str::from_utf8(&out.stdout).unwrap().trim());
    } else {
        println!("Spades [ERRORS]");
    }
}

fn run_spades(r1: &PathBuf, r2: &PathBuf) {
    let out = Command::new("spades")
        .arg("--version")
        .output()
        .unwrap();

    println!("{:?}", out.status);
}

