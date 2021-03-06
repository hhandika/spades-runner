use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::utils;

pub fn parse_seqdir(input: &str) -> Vec<SeqDirs> {
    let file = File::open(input).unwrap();
    let buff = BufReader::new(file);

    let mut seqdir = Vec::new();
    let mut lcounts: usize = 0;

    buff.lines()
        .filter_map(|ok| ok.ok())
        .skip(1)
        .for_each(|line| {
            let mut sample = SeqDirs::new();

            if line.contains(',') {
                sample.parse_csv(&line);
            } else if line.contains(':') {
                sample.parse_ini(&line);
            } else {
                panic!("INVALID INPUT FORMAT. \
                    LOOKING FOR ',' or ':' FOUND {}", line);
            }
            seqdir.push(sample);
            lcounts += 1;
        });
    
    println!("Total samples: {}", lcounts);

    seqdir
}


pub struct SeqDirs {
    pub id: PathBuf,
    pub dir: PathBuf,
}

impl SeqDirs {
    fn new() -> Self {
        Self {
            id: PathBuf::new(),
            dir: PathBuf::new(),
        }
    }

    fn parse_csv(&mut self, line: &str) {
        let sep = ',';
        let lines = utils::split_strings(line, sep);
        assert_eq!(2, lines.len());
        self.parse_samples(&lines);
    }

    fn parse_ini(&mut self, line: &str) {
        let sep = ':';
        let lines = utils::split_strings(line, sep);
        assert_eq!(2, lines.len());
        self.parse_samples(&lines);
    }

    fn parse_samples(&mut self, lines: &[String]) {
        self.id = PathBuf::from(&lines[0]);
        self.dir = PathBuf::from(&lines[1]);
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_ini_test() {
        let input = "test_files/spade_runner.ini";
        let samples = parse_seqdir(&input);

        assert_eq!(2, samples.len());
    }
}