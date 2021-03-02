use std::path::PathBuf;

use glob::glob;
use walkdir::WalkDir;


pub fn find_reads(path: &str, dirname: &str) -> Vec<SeqReads> {
    let dirs = get_dir(path, dirname);

    let mut reads: Vec<SeqReads> = Vec::new();
    dirs.iter()
        .for_each(|d| {
            let mut files = SeqReads::new(&d);
            let fastq = glob_fastq(d);
            files.match_reads(&fastq);
            reads.push(files);
        });
    
    reads
}

fn get_dir(path: &str, dirname: &str) -> Vec<String> {
    let mut entries = Vec::new();

    WalkDir::new(path).into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.file_type().is_dir())
        .for_each(|e| {
            let dir = e.path().to_string_lossy().to_string();
            if dir.contains(dirname) {
                entries.push(dir);
            }
        });
    
    entries                    
}

fn glob_fastq(path: &str) -> Vec<PathBuf> {
    let pattern = format!("{}/*.gz", path);

    glob(&pattern)
        .unwrap()
        .filter_map(|ok| ok.ok())
        .collect()
}

pub struct SeqReads {
    pub dir: PathBuf,
    pub read_1: PathBuf,
    pub read_2: PathBuf,
    pub singleton: Option<PathBuf>
}

impl SeqReads {
    fn new(dir: &str) -> Self {
        Self {
            dir: PathBuf::from(dir),
            read_1: PathBuf::new(),
            read_2: PathBuf::new(),
            singleton: None,
        }
    }

    fn match_reads(&mut self, dirs: &[PathBuf]) {
        dirs.iter()
            .for_each(|e| {
                match e.to_string_lossy().to_uppercase() {
                    d if d.contains("_READ1") => self.read_1 = PathBuf::from(e),
                    d if d.contains("_R1") => self.read_1 = PathBuf::from(e),
                    d if d.contains("_READ2") => self.read_2 = PathBuf::from(e),
                    d if d.contains("_R2") => self.read_2 = PathBuf::from(e),
                    d if d.contains("_SINGLETON") => self.singleton = Some(PathBuf::from(e)),
                    _ => (),
                }
            })
        
    }
}