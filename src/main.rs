use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::{env, fs};

fn is_file_searchable(path: &Path, extensions: &Option<Vec<&str>>) -> bool {
    match extensions {
        Some(extensions) => {
            if let Some(file_ext) = path.extension() {
                let file_ext_str = file_ext.to_string_lossy();
                extensions.iter().any(|&ext| file_ext_str == ext)
            } else {
                false
            }
        }
        None => true,
    }
}

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

fn recursive_dir_check(path: &Path, pattern: &str, extensions: &Option<Vec<&str>>) {
    for entry in fs::read_dir(path).expect("failed to read dir") {
        let entry = entry.expect("failed to read dir entry");
        let path = entry.path();

        if path.is_file() && is_file_searchable(&path, extensions) {
            find_pattern_in_file(&path, pattern);
        } else if path.is_dir() {
            recursive_dir_check(&path, pattern, extensions);
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);

    let pattern = args.next().expect("missing pattern");
    let path = PathBuf::from(args.next().expect("missing path"));

    let mut ext_raw: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--ext" => {
                let value = args.next().expect("--ext requires a value");
                ext_raw = Some(value);
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                std::process::exit(1);
            }
        }
    }

    let extensions: Option<Vec<&str>> = ext_raw.as_ref().map(|raw| {
        raw.split(',').collect::<Vec<&str>>()
    });

    if path.is_file() {
        find_pattern_in_file(&path, &pattern);
    } else if path.is_dir() {
        recursive_dir_check(&path, &pattern, &extensions);
    }
}
