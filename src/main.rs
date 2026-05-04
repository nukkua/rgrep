use crate::cli::Config;
use crate::matcher::{find_matches};

mod cli;
mod error;
mod matcher;

fn main() {
    let config = match Config::from_args() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("X {e}");
            std::process::exit(1);
        }
    };

    for file_path in &config.files {
        let file = match std::fs::File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("X {}: {}", file_path.display(), e);
                continue;
            }
        };
        let reader = std::io::BufReader::new(file);
        let matches = find_matches(reader, &config);
        for m in matches {
            if config.files.len() > 1 {
                print!("{}:", file_path.display());
            }
            if config.line_numbers {
                print!("{}:", m.line_number);
            }
            println!("{}", m.content);
        }
    }
}
