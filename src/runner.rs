use std::process::{Command};
use std::str;
use std::path::PathBuf;
use std::fs;

pub fn check_spades() {
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

#[allow(dead_code)]
pub fn run_spades(forward: &PathBuf, reverse: &PathBuf, outdir: &PathBuf) {
    fs::create_dir_all(outdir).unwrap();
    let mut out = Command::new("spades");
        
    out.arg("--pe1-1")
        .arg(forward)
        .arg("--pe1-2")
        .arg(reverse)
        .arg("-o")
        .arg(outdir);
    
    get_default_args(&mut out);
    
    println!("DONE!");
    
}

fn get_default_args(out: &mut Command) {
    out.arg("--careful");
}
