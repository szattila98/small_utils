use std::{ffi::OsString, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemPrefError {
    #[error("Could not get working dir, reason: '{0}'")]
    WorkingDirRetrievalFailed(io::Error),
    #[error("Current directory string is invalid: '{0:?}'")]
    WorkingDirParseFailed(OsString),
    #[error("File pattern is invalid: '{0}'")]
    GlobPatternInvalid(#[from] glob::PatternError),
}
