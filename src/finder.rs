use std::path::PathBuf;

use glob::{self, MatchOptions};
use walkdir::WalkDir;

use crate::parser::SeqDirs;

pub fn auto_find_cleaned_fastq(path: &str, dirname: &str)  -> Vec<SeqReads> {
    let mut entries = Vec::new();

    WalkDir::new(path).into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.file_type().is_dir())
        .for_each(|e| {
            let dir = e.path().to_string_lossy();
            if dir.contains(dirname) {
                let target = None;
                get_cleaned_fastq(&dir, &mut entries, target);
            }
        }); 
    
    entries                    
}

pub fn find_cleaned_fastq(dirs: &[SeqDirs])  -> Vec<SeqReads> {
    let mut entries = Vec::new();

    dirs.iter()
        .for_each(|s| {
            get_cleaned_fastq(&s.dir, &mut entries, Some(s.id.clone()))
        });
    
    entries
}

fn get_cleaned_fastq(
    dir: &str, 
    entries: &mut Vec<SeqReads>, 
    target: Option<String>
) {
    let mut files = SeqReads::new(&dir);
    let fastq = files.glob_fastq();
    files.match_reads(&fastq);
    files.get_id(target);

    if !files.read_1.as_os_str().is_empty() {
        entries.push(files);
    }
}

pub struct SeqReads {
    pub dir: PathBuf,
    pub id: String, 
    pub read_1: PathBuf,
    pub read_2: PathBuf,
    pub singleton: Option<PathBuf>
}

impl SeqReads {
    fn new(dir: &str) -> Self {
        Self {
            dir: PathBuf::from(dir),
            id: String::new(),
            read_1: PathBuf::new(),
            read_2: PathBuf::new(),
            singleton: None,
        }
    }

    fn glob_fastq(&self) -> Vec<PathBuf> {
        let pattern = format!("{}/*.f*.g*", 
            self.dir.to_string_lossy());
    
        let opts = MatchOptions {
            case_sensitive: true,
            ..Default::default()
        };
    
        glob::glob_with(&pattern, opts)
            .unwrap()
            .filter_map(|ok| ok.ok())
            .collect()
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
            });
    }

    fn get_id(&mut self, target: Option<String>) {
        if target.is_none() {
            let dirs: Vec<_> = self.dir.components().map(|d| d.as_os_str()).collect();
            assert!(dirs.len() > 1, "INVALID FOLDER STRUCTURE TO USE AUTO");
            self.id = String::from(dirs[1].to_string_lossy());
        } else {
            self.id = String::from(target.as_ref().unwrap());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn glob_test() {
        let input = "test_files/";

        let seq = SeqReads::new(&input);

        let res = seq.glob_fastq();
        assert_eq!(2, res.len());
    }

    #[test]
    fn find_cleaned_fastq_test() {
        let input = "test_files/";
        let dirname = "trimmed";

        let res = auto_find_cleaned_fastq(&input, &dirname);

        assert_eq!(1, res.len());
    }

    #[test]
    fn find_cleaned_fastq_reads_test() {
        let input = "test_files/";
        let dirname = "trimmed";

        let res = auto_find_cleaned_fastq(&input, &dirname);

        let path = PathBuf::from(input).join("trimmed_test");
        let r1 = path.join("some_seq_ABC123_R1.fq.gz");
        let r2 = path.join("some_seq_ABC123_R2.fq.gz");
        res.iter()
            .for_each(|e| {
                assert_eq!(r1, e.read_1);
                assert_eq!(r2, e.read_2);
                assert_eq!(String::from("trimmed_test"), e.id);
            })
    }

    #[test]
    fn get_cleaned_fastq_test() {
        let dir = "test_files/trimmed_test";
        let mut res = Vec::new();

        get_cleaned_fastq(&dir, &mut res, None);
        let path = PathBuf::from(dir);
        let r1 = path.join("some_seq_ABC123_R1.fq.gz");
        let r2 = path.join("some_seq_ABC123_R2.fq.gz");
        res.iter()
            .for_each(|e| {
                assert_eq!(r1, e.read_1);
                assert_eq!(r2, e.read_2);
            })
    }
}