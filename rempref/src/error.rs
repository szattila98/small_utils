use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemPrefError {
    #[error("File pattern is invalid: '{0}'")]
    GlobPatternInvalid(#[from] nu_glob::PatternError),
}
