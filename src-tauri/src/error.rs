use thiserror::Error;

#[derive(Error, Debug)]
pub enum JarvisError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("Platform not supported for this operation")]
    UnsupportedPlatform,

    #[error("Configuration error: {0}")]
    Config(String),
}

impl From<JarvisError> for String {
    fn from(err: JarvisError) -> Self {
        err.to_string()
    }
}
