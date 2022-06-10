use std::{ffi::OsString, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemovePrefixError {
    #[error("Could not get working dir, reason: '{0}'")]
    WorkingDirRetrieval(io::Error),
    #[error("Current directory string is invalid: '{0:?}'")]
    WorkingDirParse(OsString),
    #[error("File pattern is invalid: '{0}'")]
    GlobPattern(#[from] glob::PatternError),
}
