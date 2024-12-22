#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    CliError(clap::Error),
    UnknownError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CliError(error) => write!(f, "cli error: {}", error),
            Error::UnknownError(msg) => write!(f, "unknown error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
