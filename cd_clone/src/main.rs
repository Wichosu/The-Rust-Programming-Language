use std::env;
use std::io;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Only one argument can be taken");
        process::exit(0);
    } 

    let directory_path = &args[1];

    let path = Path::new(directory_path);

    if path.exists() && path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        println!("{}", file_name);
                    }
                }
            }
        }
    }
}
