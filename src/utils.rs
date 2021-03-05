use std::fs;
use std::path::Path;
use std::io::{self, Result, Write};

pub fn check_dir_exists(dir: &Path) {
    if dir.exists() {
        panic!("{:?} DIR EXISTS. PLEASE RENAME OR REMOVE IT", dir);
    } else { // if not create one
        fs::create_dir_all(dir)
            .expect("CAN'T CREATE CLEAN READ DIR");
    }
}

pub fn print_done() -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "\x1b[0;32mDONE!\x1b[0m")?;

    Ok(())
}