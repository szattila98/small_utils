#![allow(clippy::derive_ord_xor_partial_ord)]

use super::traits::{Relativize, ToFailed};
use pathdiff::diff_paths;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct FileOperationTask {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl FileOperationTask {
    pub fn new(from: PathBuf, to: PathBuf) -> Self {
        Self { from, to }
    }
}

impl ToFailed for FileOperationTask {
    fn to_failed(&self, reason: &str) -> FailedFileOperation {
        FailedFileOperation::new(self.from.clone(), reason.to_string())
    }
}

impl Relativize for FileOperationTask {
    fn relativize(&self, working_dir: &Path) -> Self {
        let mut task = self.clone();
        task.from = diff_paths(&task.from, working_dir).unwrap();
        task.to = diff_paths(&task.to, working_dir).unwrap();
        task
    }
}

impl Display for FileOperationTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from.display(), self.to.display())
    }
}

impl Ord for FileOperationTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.from
            .display()
            .to_string()
            .cmp(&other.from.display().to_string())
            .cmp(&self.from.iter().count().cmp(&other.from.iter().count()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct FailedFileOperation {
    pub file_path: PathBuf,
    pub reason: String,
}

impl FailedFileOperation {
    pub fn new(file_path: PathBuf, reason: String) -> Self {
        Self { file_path, reason }
    }
}

impl Relativize for FailedFileOperation {
    fn relativize(&self, working_dir: &Path) -> Self {
        let mut task = self.clone();
        task.file_path = diff_paths(task.file_path, working_dir).unwrap();
        task
    }
}

impl Display for FailedFileOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.file_path.display(), self.reason)
    }
}

impl Ord for FailedFileOperation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.file_path
            .display()
            .to_string()
            .cmp(&other.file_path.display().to_string())
            .cmp(
                &self
                    .file_path
                    .iter()
                    .count()
                    .cmp(&other.file_path.iter().count()),
            )
    }
}

pub struct FileOperationResult {
    pub successful: usize,
    pub failed: usize,
}

impl FileOperationResult {
    pub fn new(successful: usize, failed: usize) -> Self {
        Self { successful, failed }
    }
}

impl<T: Relativize> Relativize for Vec<T> {
    fn relativize(&self, working_dir: &Path) -> Self {
        self.iter()
            .map(|task| task.relativize(working_dir))
            .collect::<Vec<_>>()
    }
}

impl ToFailed for PathBuf {
    fn to_failed(&self, reason: &str) -> FailedFileOperation {
        FailedFileOperation::new(self.clone(), reason.to_string())
    }
}
