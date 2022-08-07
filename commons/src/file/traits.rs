use super::{
    errors::FileOperationError,
    model::{FailedFileOperation, FileOperationResult, FileOperationTask},
};
use std::path::{Path, PathBuf};

pub trait Relativizable {
    fn relativize(&self, working_dir: &Path) -> Self;
}

pub trait ToFileTask: IntoIterator + Sized {
    fn to_file_tasks<T>(self, task_generator: T) -> Vec<FileOperationTask>
    where
        T: Fn(<Self as IntoIterator>::Item) -> FileOperationTask,
    {
        self.into_iter()
            .map(task_generator)
            .collect::<Vec<FileOperationTask>>()
    }
}

impl ToFileTask for Vec<PathBuf> {}

pub trait FileOperation {
    fn get_tasks(&self) -> Vec<FileOperationTask>;

    fn get_failed_tasks(&self) -> Vec<FailedFileOperation>;

    fn execute(&mut self) -> Result<FileOperationResult, FileOperationError>;
}
