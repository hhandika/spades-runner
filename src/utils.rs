use std::fs;
use std::path::Path;
use std::io::{self, Result, Write};

use sysinfo::SystemExt;

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

pub fn split_strings(lines: &str, sep: char) -> Vec<String> {
    lines.split(sep)
        .map(|e| e.trim().to_string())
        .collect()
}

pub fn get_system_info() -> Result<()> {
    let mut sysinfo = sysinfo::System::new_all();
    sysinfo.refresh_all();

    let io = io::stdout();
    let mut handle = io::BufWriter::new(io);

    let total_ram = sysinfo.get_total_memory();
    let gb = 1048576;

    writeln!(handle, "\x1b[0;33mSystem Information\x1b[0m")?;

    writeln!(handle, "Operating system\t: {} {}", 
        sysinfo.get_name().as_ref().unwrap(),
        sysinfo.get_os_version().as_ref().unwrap())?;

    writeln!(handle, "Kernel version\t\t: {}", sysinfo.get_kernel_version().as_ref().unwrap())?;
    writeln!(handle, "Available cores\t\t: {:?}", num_cpus::get_physical())?;
    writeln!(handle, "Available threads\t: {:?}", num_cpus::get())?;
    writeln!(handle, "Total RAM\t\t: {} Gb", total_ram/gb)?;
    writeln!(handle)?;

    Ok(())
}