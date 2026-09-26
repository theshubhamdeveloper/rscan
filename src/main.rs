use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, fs};

fn find_pattern_in_file(path: &Path, pattern: &str) {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_else(|_err| String::new());

        if line.contains(pattern) {
            println!("{}:{}: {}", path.to_string_lossy(), index + 1, line)
        };
    }
}

fn recursive_dir_check(path: &Path, pattern: &str) {
    for entry in fs::read_dir(path).expect("failed to read dir") {
        let entry = entry.expect("failed to read dir entry");
        let path = entry.path();

        if path.is_file() {
            find_pattern_in_file(&path, pattern);
        } else if path.is_dir() {
            recursive_dir_check(&path, pattern);
        }
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
        recursive_dir_check(path, pattern);
    }
}
