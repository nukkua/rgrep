use std::io::BufRead;

use crate::cli::Config;

pub struct Match {
    pub line_number: usize,
    pub content: String,
}

impl Match {
    pub fn find_matches(reader: impl BufRead, config: &Config) -> Vec<Match> {
        let mut matches = Vec::new();

        let pattern = if config.case_insensitive {
            config.pattern.to_lowercase()
        } else {
            config.pattern.clone()
        };

        for (index, line_result) in reader.lines().enumerate() {
            let line = match line_result {
                Ok(line) => line,
                Err(_) => continue,
            };

            let line_to_search = if config.case_insensitive {
                line.to_lowercase()
            } else {
                line.clone()
            };

            if line_to_search.contains(&pattern) {
                matches.push(Match {
                    line_number: index + 1,
                    content: line,
                });
            }
        }

        matches
    }
}
