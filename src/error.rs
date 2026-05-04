use std::fmt;

#[derive(Debug)]
pub enum GrepError {
    Io(std::io::Error),
    InvalidPattern(String),
    MissingArgument(String),
}

impl fmt::Display for GrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrepError::Io(e) => write!(f, "io error: {e}"),
            GrepError::InvalidPattern(msg) => write!(f, "{msg}"),
            GrepError::MissingArgument(msg) => write!(f, "missing argument: {msg}"),
        }
    }
}

impl std::error::Error for GrepError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GrepError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for GrepError {
    fn from(e: std::io::Error) -> Self {
        GrepError::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, GrepError>;
