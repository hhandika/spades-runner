use std::io::{self, Result, Write};
use std::os::unix;
use std::process::{Command, Output};
use std::str;
use std::path::{Path, PathBuf};

use spinners::{Spinner, Spinners};

use crate::finder::SeqReads;
use crate::utils;

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

pub fn assemble_reads(reads: &[SeqReads]) {
    let dir = Path::new("assemblies");
    utils::check_dir_exists(&dir);
    reads.iter()
        .for_each(|r| {
            println!("\x1b[0;33m================Processing {}================\x1b[0m", &r.target_dir.to_string_lossy());
            let mut run = Runner::new(&dir, r);
            run.display_settings().unwrap();
            run.run_spades();
        })
}

struct Runner<'a> {
    reads: &'a SeqReads,
    output: PathBuf,
    symlink_dir: PathBuf, 
}

impl<'a> Runner<'a> {
    fn new(dir: &Path, input: &'a SeqReads) -> Self {
        Self {
            reads: input,
            output: dir.join(&input.target_dir),
            symlink_dir: dir.join("contig_symlinks"),
        }
    }

    fn run_spades(&mut self) {
        let spin = self.set_spinner();
        let out = self.call_spades();
        self.check_spades_success(&out);
        spin.stop();
        utils::print_done().unwrap();
        self.create_symlink();
    }

    fn check_spades_success(&self, out: &Output) {
        if !out.status.success() {
            io::stdout().write_all(&out.stdout).unwrap();
            io::stdout().write_all(&out.stderr).unwrap();
        }
    }

    fn call_spades(&self) -> Output {
        let mut out = Command::new("spades");
            
        out.arg("--pe1-1")
            .arg(&self.reads.read_1)
            .arg("--pe1-2")
            .arg(&self.reads.read_2)
            .arg("-o")
            .arg(&self.output.clone());
        
        self.get_default_args(&mut out);
        out.output().unwrap()
    }
    
    fn get_default_args(&self, out: &mut Command) {
        out.arg("--careful");
    }
 
    fn set_spinner(&mut self) -> Spinner {
        let msg = "SPAdes is processing...\t".to_string();
        
        Spinner::new(Spinners::Moon, msg)
    }

    fn display_settings(&self) -> Result<()> {
        let stdout = io::stdout();
        let mut buff = io::BufWriter::new(stdout);
        
        writeln!(buff, "Target dir\t: {}", &self.reads.target_dir.to_string_lossy())?;
        writeln!(buff, "Input R1\t: {}", &self.reads.read_1.to_string_lossy())?;
        writeln!(buff, "Input R2\t: {}", &self.reads.read_2.to_string_lossy())?;
        writeln!(buff, "Output\t\t: {}", &self.output.to_string_lossy())?;
        writeln!(buff)?;

        Ok(())
    }

    fn create_symlink(&self) {
        let contigs_path = self.output.join("contigs.fasta");

        if contigs_path.is_file() {
            let path = contigs_path.canonicalize().unwrap();
            let symlink = self.symlink_dir.join(path.file_name().unwrap());
            unix::fs::symlink(path, symlink).unwrap();
        } else {
            println!("A contig file is not found.");
        }
    }
}
