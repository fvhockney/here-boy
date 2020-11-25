use std::error::Error;
use std::fmt;
use std::path::PathBuf;

pub enum MockError {
    UnparsableUri(String),
    UnableToGet,
    UnableToCreateFile(PathBuf),
    UnableToWriteToFile(PathBuf),
    NoChunk,
    RequestFailed(String, String),
    NoConfigFound(PathBuf),
    CantCreatePaths(PathBuf),
    MalformedConfig(PathBuf),
}

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MockError::UnparsableUri(uri) => write!(f, "unable to parse {} as a uri", uri),
            MockError::UnableToGet => write!(f, "unable to initiate get request"),
            MockError::UnableToCreateFile(file) => {
                write!(f, "unable to create file {}", file.display())
            }
            MockError::UnableToWriteToFile(file) => {
                write!(f, "unable to write to file {}", file.display())
            }
            MockError::NoChunk => write!(f, "no chunk"),
            MockError::RequestFailed(uri, status) => {
                write!(f, "request to {} failed with {}", uri, status)
            }
            MockError::NoConfigFound(path) => write!(f, "no config found at {}", path.display()),
            MockError::CantCreatePaths(path) => {
                write!(f, "can not create path for {}", path.display())
            }
            MockError::MalformedConfig(path) => write!(f, "malformed config at {}", path.display()),
        }
    }
}

impl fmt::Debug for MockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MockError::UnparsableUri(uri) => write!(f, "unable to parse {} as a uri", uri),
            MockError::UnableToGet => write!(f, "unable to initiate get request"),
            MockError::UnableToCreateFile(file) => {
                write!(f, "unable to create file {}", file.display())
            }
            MockError::UnableToWriteToFile(file) => {
                write!(f, "unable to write to file {}", file.display())
            }
            MockError::NoChunk => write!(f, "no chunk"),
            MockError::RequestFailed(uri, status) => {
                write!(f, "request to {} failed with {}", uri, status)
            }
            MockError::NoConfigFound(path) => write!(f, "no config found at {}", path.display()),
            MockError::CantCreatePaths(path) => {
                write!(f, "can not create path for {}", path.display())
            }
            MockError::MalformedConfig(path) => write!(f, "malformed config at {}", path.display()),
        }
    }
}

impl Error for MockError {}
