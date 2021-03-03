use std::path::PathBuf;

use glob::{self, MatchOptions};
use walkdir::WalkDir;

pub fn find_cleaned_fastq(path: &str, dirname: &str)  -> Vec<SeqReads> {
    let mut entries = Vec::new();

    WalkDir::new(path).into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.file_type().is_dir())
        .for_each(|e| {
            let dir = e.path().to_string_lossy();
            if dir.contains(dirname) {
                let fastq = glob_fastq(&dir);
                let mut files = SeqReads::new(&dir);
                files.match_reads(&fastq);
                files.get_target_dir();
                if !files.read_1.as_os_str().is_empty() {
                    entries.push(files);
                }
            }
        }); 
    
    entries                    
}

fn glob_fastq(path: &str) -> Vec<PathBuf> {
    let pattern = format!("{}/*.f*.g*", path);

    let opts = MatchOptions {
        case_sensitive: true,
        ..Default::default()
    };

    glob::glob_with(&pattern, opts)
        .unwrap()
        .filter_map(|ok| ok.ok())
        .collect()
}

pub struct SeqReads {
    pub dir: PathBuf,
    pub target_dir: PathBuf, 
    pub read_1: PathBuf,
    pub read_2: PathBuf,
    pub singleton: Option<PathBuf>
}

impl SeqReads {
    fn new(dir: &str) -> Self {
        Self {
            dir: PathBuf::from(dir),
            target_dir: PathBuf::new(),
            read_1: PathBuf::new(),
            read_2: PathBuf::new(),
            singleton: None,
        }
    }

    fn get_target_dir(&mut self) {
        let dirs: Vec<_> = self.dir.components().map(|d| d.as_os_str()).collect();

        self.target_dir = PathBuf::from(dirs[1]);
    }

    fn match_reads(&mut self, dirs: &[PathBuf]) {
        dirs.iter()
            .for_each(|e| {
                match e.to_string_lossy().to_uppercase() {
                    d if d.contains("READ1") => self.read_1 = PathBuf::from(e),
                    d if d.contains("R1") => self.read_1 = PathBuf::from(e),
                    d if d.contains("READ2") => self.read_2 = PathBuf::from(e),
                    d if d.contains("R2") => self.read_2 = PathBuf::from(e),
                    d if d.contains("SINGLETON") => self.singleton = Some(PathBuf::from(e)),
                    _ => (),
                }
            })
        
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn glob_test() {
        let input = "test_files/";

        let res = glob_fastq(&input);
        assert_eq!(2, res.len());
    }
}