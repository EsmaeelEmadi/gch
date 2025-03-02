use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ReadConfigError {
    CurrentDirError(std::io::Error),
    FileOpenError {
        path: PathBuf,
        source: std::io::Error,
    },
    DeserializeError(serde_yml::Error),
}

impl fmt::Display for ReadConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadConfigError::CurrentDirError(e) => {
                write!(f, "Failed to get current directory: {}", e)
            }
            ReadConfigError::FileOpenError { path, source } => {
                write!(f, "Failed to open file {:?}: {}", path, source)
            }
            ReadConfigError::DeserializeError(e) => {
                write!(f, "Failed to deserialize config: {}", e)
            }
        }
    }
}

impl std::error::Error for ReadConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReadConfigError::CurrentDirError(e) => Some(e),
            ReadConfigError::FileOpenError { source, .. } => Some(source),
            ReadConfigError::DeserializeError(e) => Some(e),
        }
    }
}
