#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    CliError(clap::Error),
    IOError(std::io::Error),
    SerializationError(serde_json::Error),
    UnknownError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CliError(error) => write!(f, "cli error: {}", error),
            Error::IOError(error) => write!(f, "i/o error: {}", error),
            Error::SerializationError(error) => write!(f, "serialization error: {}", error),
            Error::UnknownError(msg) => write!(f, "unknown error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<clap::Error> for Error {
    fn from(value: clap::Error) -> Self {
        Self::CliError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializationError(value)
    }
}
