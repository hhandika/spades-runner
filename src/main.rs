use std::process::Command;
use std::str;
use std::path::PathBuf;
use std::fs;

fn main() {
    let r1 = PathBuf::from("data/sample_buno_r1.fastq.gz");
    let r2 = PathBuf::from("data/sample_buno_clean_r1.fastq.gz");
    let outdir = PathBuf::from("data/spades");
    check_spades();
    // test_spades();
    run_spades(&r1, &r2, &outdir);

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


fn run_spades(forward: &PathBuf, reverse: &PathBuf, outdir: &PathBuf) {
    fs::create_dir_all(outdir).unwrap();
    let mut out = Command::new("spades")
        .arg("--pe1-1")
        .arg(forward)
        .arg("--pe1-2")
        .arg(reverse)
        .arg("--careful")
        .arg("-o")
        .arg(outdir)
        .spawn()
        .unwrap();
    
    out.wait().unwrap();
    
    println!("DONE!");
    
}

