use std::fs;
use std::path::{Path, PathBuf};

use glob::glob;
use walkdir::WalkDir;

pub fn clean_spades_files(path: &Path) {
    WalkDir::new(path).into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.path().ends_with("contigs.fasta"))
        .for_each(|e| {
            let path = e.path().parent().unwrap().to_string_lossy();
            let patterns = format!("{}/*", path);
            let contents = glob(&patterns)
                .unwrap()
                .filter_map(|ok| ok.ok())
                .collect::<Vec<PathBuf>>();
            remove_contents(&contents);
        });
}

fn remove_contents(contents: &[PathBuf]) {
    contents.iter()
        .for_each(|e| {
            if e.is_file() {
                match e.to_string_lossy() {
                    p if p.ends_with("/contigs.fasta") => (),
                    p if p.ends_with("/scaffolds.fasta") => (),
                    p if p.ends_with("/spades.log") => (),
                    p if p.ends_with("/warnings.log") => (),
                    _ => fs::remove_file(e).unwrap(),
                }
            } 
            if e.is_dir() {
                fs::remove_dir_all(e).unwrap();
            }     
        });
}