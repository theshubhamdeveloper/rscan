use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, fs};

fn find_pattern_in_file(path: &Path, pattern: &String) {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");

        if line.contains(pattern) {
            println!("{}:{}: {}", path.to_string_lossy(), index + 1, line)
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <path>", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];
    let path = Path::new(&args[2]);

    if path.is_file() {
        find_pattern_in_file(path, pattern);
    } else {
        for entry in fs::read_dir(path).expect("failed to read dir") {
            let entry = entry.expect("failed to read dir entry");
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            find_pattern_in_file(path.as_path(), pattern);
        }
    }
}
