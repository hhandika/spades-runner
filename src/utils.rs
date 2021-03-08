use std::fs;
use std::path::Path;
use std::io::{self, Result, Write};

use sysinfo::SystemExt;
use chrono::NaiveTime;

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

fn parse_duration(duration: u64) -> String {
    let sec = (duration % 60) as u32;
    let min = ((duration/60) % 60) as u32;
    let hours = ((duration/60) / 60) as u32;
    let time = NaiveTime::from_hms(hours, min, sec);
    
    time.format("%H:%M:%S").to_string()
}

pub fn print_formatted_duration(duration: u64) {
    let time = parse_duration(duration);
    println!("Execution time (HH:MM:SS): {}", time);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn time_parsing_test() {
        let duration = 65;
        let duration_2 = 3600;
        let time = parse_duration(duration);
        let hours = parse_duration(duration_2);

        assert_eq!("00:01:05", time);
        assert_eq!("01:00:00", hours);
    }
}