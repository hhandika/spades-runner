use crate::finder;

pub fn auto_find_reads(path: &str, dirname: &str) {
    let dirs = finder::find_reads(path, dirname);

    println!("\x1b[0;33mDIR LIST:\n\x1b[0m");

    dirs.iter()
        .for_each(|e| {
            println!("Dir\t\t: {}", e.dir.to_string_lossy());
            println!("Read 1\t\t: {}", e.read_1.to_string_lossy());
            println!("Read 2\t\t: {}", e.read_2.to_string_lossy());

            if e.singleton.is_some() {
                println!("Singleton\t: {}", e.singleton.as_ref().unwrap().to_string_lossy());
            }

            println!();
        });
}