use std::io::{self, Result, Write};

use crate::finder::{self, SeqReads};

pub fn auto_find_reads(path: &str, dirname: &str) {
    let dirs = finder::find_cleaned_fastq(path, dirname);
    display_dryrun(&dirs).unwrap();
}

pub fn dry_run(path: &str, dirname: &str) {
    let dirs = finder::find_cleaned_fastq(path, dirname);
    display_dryrun(&dirs).unwrap();
}

pub fn display_dryrun(dirs: &[SeqReads]) -> Result<()> {
    let out = io::stdout();
    let mut handle = io::BufWriter::new(out);

    writeln!(handle,"\x1b[0;33mDIR LIST:\n\x1b[0m")?;

    dirs.iter()
        .for_each(|e| {
            writeln!(handle,"\x1b[0;34mOrigin\t\t: {}\x1b[0m", e.dir.to_string_lossy()).unwrap();
            writeln!(handle,"Read 1\t\t: {}", e.read_1.to_string_lossy()).unwrap();
            writeln!(handle,"Read 2\t\t: {}", e.read_2.to_string_lossy()).unwrap();

            if e.singleton.is_some() {
                writeln!(handle,"Singleton\t: {}", e.singleton.as_ref().unwrap().to_string_lossy()).unwrap();
            }

            writeln!(handle,"Target dir\t: {}", e.target_dir.to_string_lossy()).unwrap();

            writeln!(handle).unwrap();
        });
    
    Ok(())
}