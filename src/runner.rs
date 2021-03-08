use std::fs;
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

pub fn assemble_reads(reads: &[SeqReads], threads: Option<usize>) {
    let dir = Path::new("assemblies");
    utils::check_dir_exists(&dir);
    let contig_dir = dir.join("contig_symlinks");
    fs::create_dir_all(&contig_dir).unwrap();

    reads.iter()
        .for_each(|r| {
            println!("\x1b[0;33m================Processing {}================\x1b[0m", &r.id.to_string_lossy());
            let mut run = Runner::new(&dir, &contig_dir, r, threads);
            run.run_spades();
        })
}

struct Runner<'a> {
    reads: &'a SeqReads,
    output: PathBuf,
    symlink_dir: &'a Path,
    threads: Option<usize>, 
}

impl<'a> Runner<'a> {
    fn new(
        dir: &Path, 
        contig_dir: &'a Path, 
        input: &'a SeqReads, 
        threads: Option<usize>
    ) -> Self {
        Self {
            reads: input,
            output: dir.join(&input.id),
            symlink_dir: contig_dir,
            threads,
        }
    }

    fn run_spades(&mut self) {
        self.print_settings().unwrap();
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

        if self.reads.singleton.is_some() {
            self.get_singleton(&mut out);
        }

        if self.threads.is_some() {
            self.get_thread_num(&mut out);
        }

        out.output().unwrap()
    }
    
    fn get_default_args(&self, out: &mut Command) {
        out.arg("--careful");
    }

    fn get_singleton(&self, out: &mut Command) {
        out.arg("--pe1-s")
            .arg(self.reads.singleton.as_ref().unwrap());
    }

    fn get_thread_num(&self, out: &mut Command) {
        out.arg("--threads")
            .arg(self.threads.as_ref().unwrap().to_string());
    }
 
    fn set_spinner(&mut self) -> Spinner {
        let msg = "SPAdes is processing...\t".to_string();
        
        Spinner::new(Spinners::Moon, msg)
    }

    fn print_settings(&self) -> Result<()> {
        let stdout = io::stdout();
        let mut buff = io::BufWriter::new(stdout);
        
        writeln!(buff, "ID\t\t: {}", &self.reads.id.to_string_lossy())?;
        writeln!(buff, "Input R1\t: {}", &self.reads.read_1.to_string_lossy())?;
        writeln!(buff, "Input R2\t: {}", &self.reads.read_2.to_string_lossy())?;

        if self.reads.singleton.is_some() {
            writeln!(buff, "Singleton\t: {}", 
                &self.reads.singleton.as_ref().unwrap().to_string_lossy())?;
        }

        writeln!(buff, "Output\t\t: {}", &self.output.to_string_lossy())?;
        writeln!(buff)?;

        Ok(())
    }

    fn create_symlink(&self) {
        let contig_sym = format!("{}_contigs.fasta", self.reads.id.to_string_lossy());
        let contigs_path = self.output.join("contigs.fasta");

        if contigs_path.is_file() {
            let path = contigs_path.canonicalize().unwrap();
            let symlink = self.symlink_dir.join(contig_sym);
            unix::fs::symlink(&path, &symlink).unwrap();
            self.print_contig_path(&contigs_path, &symlink).unwrap();
        } else {
            println!("A contig file is not found. \
                SPAdes may have failed to run.");
        }
    }

    fn print_contig_path(&self, path: &Path, symlink: &Path) -> Result<()>{
        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout);

        writeln!(handle)?;
        writeln!(handle, "\x1b[1mContig Path\x1b[0m")?;
        writeln!(handle, "File\t\t: {}", path.to_string_lossy())?;
        writeln!(handle, "Symlink\t\t: {}", symlink.to_string_lossy())?;
        writeln!(handle)?;

        Ok(())
    }
}
