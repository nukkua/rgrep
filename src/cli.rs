use crate::error::{GrepError, Result};
use std::path::PathBuf;

pub struct Config {
    pub pattern: String,
    pub files: Vec<PathBuf>,
    pub case_insensitive: bool,
    pub line_numbers: bool,
}

impl Config {
    pub fn from_args() -> Result<Self> {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let mut files: Vec<PathBuf> = Vec::new();
        let mut pattern: Option<String> = None;
        let mut case_insensitive = false;
        let mut line_numbers = false;
        for arg in &args {
            match arg.as_str() {
                "-i" | "--insensitive" => case_insensitive = true,
                "-n" | "--lines" => line_numbers = true,
                flag if flag.starts_with('-') => {
                    return Err(GrepError::InvalidPattern(format!("unknown flag: {flag}")));
                }
                _ => {
                    if pattern.is_none() {
                        pattern = Some(arg.clone());
                    } else {
                        files.push(PathBuf::from(arg));
                    }
                }
            }
        }
        let pattern =
            pattern.ok_or_else(|| GrepError::InvalidPattern("no pattern provided".to_string()))?;
        if files.is_empty() {
            return Err(GrepError::MissingArgument("no files provided".to_string()));
        }

        Ok(Config {
            pattern,
            files,
            case_insensitive,
            line_numbers,
        })
    }
}
