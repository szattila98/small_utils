use super::model::FailedFileOperation;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckBeforeError {
    #[error("Some files would be overwritten")]
    FilesWouldOwerwrite(Vec<FailedFileOperation>),
}
