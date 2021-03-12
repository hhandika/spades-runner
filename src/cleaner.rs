use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use glob::glob;
use walkdir::WalkDir;

pub fn clean_spades_files(path: &Path) {
    let io = io::stdout();
    let mut handle = io::BufWriter::new(io);
    writeln!(handle, "\x1b[0;33mRemoved files and directories:\x1b[0m").unwrap();
    WalkDir::new(path).into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.path().ends_with("spades.log"))
        .for_each(|e| {
            let path = e.path().parent().unwrap().to_string_lossy();
            let contents = find_files(&path);
            remove_contents(&contents, &mut handle);
        });
    writeln!(handle).unwrap();
}

fn find_files(path: &str) -> Vec<PathBuf> {
    let patterns = format!("{}/*", path);

    glob(&patterns)
        .unwrap()
        .filter_map(|ok| ok.ok())
        .collect()
}

fn remove_contents<W: Write>(contents: &[PathBuf], handle: &mut W) {
    contents.iter()
        .for_each(|e| {
            if e.is_file() {
                match e.to_string_lossy() {
                    p if p.ends_with("/contigs.fasta") => (),
                    p if p.ends_with("/scaffolds.fasta") => (),
                    p if p.ends_with("/spades.log") => (),
                    p if p.ends_with("/warnings.log") => (),
                    _ => {
                        fs::remove_file(e).unwrap();
                        writeln!(handle, "{}", e.to_string_lossy()).unwrap();
                    },
                }
            } 
            if e.is_dir() {
                fs::remove_dir_all(e).unwrap();
                writeln!(handle, "{}", e.to_string_lossy()).unwrap();
            }     
        });

}