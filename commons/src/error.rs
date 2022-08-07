use thiserror::Error;

use crate::FileOperationTask;

#[derive(Debug, Error)]
pub enum FileOperationError {
    #[error("Some files would be overwritten")]
    FilesWouldOwerwrite(Vec<FileOperationTask>),
}
