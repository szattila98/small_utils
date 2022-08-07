use super::model::FileOperationTask;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileOperationError {
    #[error("Some files would be overwritten")]
    FilesWouldOwerwrite(Vec<FileOperationTask>),
}
