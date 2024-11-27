use std::env;
use std::fs::{self};
use std::io::{self, Read};
use std::path::Path;
use std::time::Instant; 

const RESET: &str = "\x1b[0m"; 
const RED: &str = "\x1b[31m";    
const GREEN: &str = "\x1b[32m";  
const YELLOW: &str = "\x1b[33m";  
const CYAN: &str = "\x1b[36m";    
const MAGENTA: &str = "\x1b[35m"; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 4 {
        eprintln!("Usage: nano-grep <pattern> [directory] [--format=<ext1,ext2,...>]");
        std::process::exit(1);
    }

    let pattern = &args[1];
    let directory = args.get(2).map_or(".", |dir| dir);  // Default to current directory if no directory is provided
    let formats = get_file_formats(&args);

    if pattern.is_empty() {
        eprintln!("{}Error: Search pattern cannot be empty.{}", RED, RESET);
        std::process::exit(1);
    }
    println!("{}Searching for '{}' in directory '{}{}'.", YELLOW, pattern, directory, RESET);

    let start_time = Instant::now();
    let matched_files_count = search_in_filesystem(pattern, directory, formats)?;
    let elapsed_time = start_time.elapsed();

    println!("\n{}Search completed in {:?}{}", GREEN, elapsed_time, RESET);
    println!("{}Found pattern in {} file(s).{}", CYAN, matched_files_count, RESET);

    Ok(())
}

fn get_file_formats(args: &[String]) -> Option<Vec<String>> {
    for arg in args.iter() {
        if arg.starts_with("--format=") {
            let formats = arg.strip_prefix("--format=").unwrap_or("");
            let extensions: Vec<String> = formats.split(',')
                .map(|ext| ext.to_string())
                .collect();
            return Some(extensions);
        }
    }
    None
}

fn search_in_filesystem(pattern: &str, directory: &str, formats: Option<Vec<String>>) -> io::Result<usize> {
    let start_path = Path::new(directory);
    let mut matched_files_count = 0;

    traverse_directory(start_path, pattern, &formats, &mut matched_files_count)?;

    Ok(matched_files_count)
}

fn traverse_directory(path: &Path, pattern: &str, formats: &Option<Vec<String>>, matched_files_count: &mut usize) -> io::Result<()> {
    if path.is_dir() {
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                traverse_directory(&entry_path, pattern, formats, matched_files_count)?;
            } else if entry_path.is_file() {
                if let Some(ref ext) = formats {
                    if !matches_extension(&entry_path, ext) {
                        continue;
                    }
                }

                if let Some((filename_match, content_match, match_count)) = match_pattern(&entry_path, pattern)? {
                    *matched_files_count += 1;
                    println!("\n[RESULT] -> {}", entry_path.display());

                    if filename_match {
                        println!("{}  ➡Filename match found:", MAGENTA);
                    }

                    if content_match {
                        println!("{}  ➡ Content match found: {} time(s){}", MAGENTA, match_count, RESET);
                    }

                    if filename_match && content_match {
                        println!("  📝 Both filename and content match found in this file!");
                    }

                    println!("----------------------------------------");
                }
            }
        }
    }
    Ok(())
}

fn matches_extension(path: &Path, formats: &[String]) -> bool {
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            return formats.contains(&ext_str.to_lowercase());
        }
    }
    false
}

fn match_pattern(path: &Path, pattern: &str) -> io::Result<Option<(bool, bool, usize)>> {
    let mut filename_match = false;
    let mut content_match = false;
    let mut match_count = 0;

    if path.file_name().unwrap_or_default().to_str().unwrap_or_default().contains(pattern) {
        filename_match = true;
    }

    match read_file_and_search(path, pattern) {
        Ok(count) if count > 0 => {
            content_match = true;
            match_count = count;
        },
        Ok(_) => {},
        Err(err) => {
            if err.kind() != io::ErrorKind::InvalidData {
                return Err(err);
            }
        },
    }

    if filename_match || content_match {
        Ok(Some((filename_match, content_match, match_count)))
    } else {
        Ok(None)
    }
}

fn read_file_and_search(path: &Path, pattern: &str) -> io::Result<usize> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let count = contents.matches(pattern).count();
    Ok(count)
}