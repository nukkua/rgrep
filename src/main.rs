use std::fs::File;
use std::io::BufReader;

use crate::cli::Config;
use crate::matcher::Match;

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
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("X {}: {}", file_path.display(), e);
                continue;
            }
        };
        let reader = BufReader::new(file);
        let matches = Match::find_matches(reader, &config);
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
