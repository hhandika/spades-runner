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

pub fn print_header(text: &str) {
    let header = format!("Processing {}", text);
    let length = 98;
    let sym = '=';
    let mut header = PrettyHeader::new(&header, sym, length);
    header.print_header().unwrap();
}

pub struct PrettyHeader {
    text: String,
    sym: char,
    len: usize,
    text_len: usize,
    sym_len: usize,
    color: String,
}

impl PrettyHeader {
    fn new(text: &str, sym: char, len: usize) -> Self {
        Self {
            text: String::from(text),
            sym, 
            len,
            text_len: 0,
            sym_len: 0,
            color: String::from("\x1b[0;33m"), 
        }
    }

    fn print_header(&mut self) -> Result<()> {
        self.get_len();
        let io = io::stdout();
        let mut handle = io::BufWriter::new(io);
        write!(handle,"{}", self.color)?;
        
        if self.text_len > self.len {
            writeln!(handle, "{}", self.text)?;
        } else {
            self.print_with_symbol(&mut handle)?;
        }
        write!(handle,"\x1b[0m")?;
        Ok(())
    }

    fn print_with_symbol<W: Write>(&mut self, handle: &mut W) -> Result<()> {
        self.print_symbols(handle);
        write!(handle, " {} ", self.text)?;
        self.print_symbols(handle);

        if self.text_len % 2 != 0 {
            write!(handle,"{}", self.sym)?;
        }

        writeln!(handle)?;
        Ok(())
    }

    fn get_len(&mut self) {
        self.text_len = self.text.len();
        self.sym_len = (self.len - self.text_len) / 2;
    }

    fn print_symbols<W: Write>(&self, io: &mut W) {
        (0..=self.sym_len).for_each(|_| {
            write!(io, "{}", self.sym).unwrap();
        });
    }
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