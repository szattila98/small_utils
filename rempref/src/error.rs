use std::{ffi::OsString, io};
use thiserror::Error;
use wax::BuildError;

#[derive(Debug, Error)]
pub enum RemPrefError {
    #[error("Could not get working dir, reason: '{0}'")]
    WorkingDirRetrievalFailed(io::Error),
    #[error("Current directory string is invalid: '{0:?}'")]
    WorkingDirParseFailed(OsString),
    #[error("File pattern is invalid: '{0}'")]
    GlobPatternInvalid(String),
}

impl From<BuildError<'_>> for RemPrefError {
    fn from(e: BuildError) -> Self {
        Self::GlobPatternInvalid(e.to_string())
    }
}
