use pathdiff::diff_paths;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub mod error;

pub trait Relativizable {
    fn relativize(&self, working_dir: &PathBuf) -> Self;
}

#[derive(Debug, Clone)]
pub struct FileOperationTask {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl FileOperationTask {
    pub fn new(from: PathBuf, to: PathBuf) -> Self {
        Self { from, to }
    }
}

impl Relativizable for FileOperationTask {
    fn relativize(&self, working_dir: &PathBuf) -> Self {
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

#[derive(Debug, Clone)]
pub struct FailedFileOperation {
    pub file_path: PathBuf,
    pub reason: String,
}

impl FailedFileOperation {
    pub fn new(file_path: PathBuf, reason: String) -> Self {
        Self { file_path, reason }
    }
}

impl Relativizable for FailedFileOperation {
    fn relativize(&self, working_dir: &PathBuf) -> Self {
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

impl<T: Relativizable> Relativizable for Vec<T> {
    fn relativize(&self, working_dir: &PathBuf) -> Self {
        self.iter()
            .map(|task| task.relativize(working_dir))
            .collect::<Vec<_>>()
    }
}

pub fn walkdir(root: &Path, depth: Option<usize>) -> Vec<PathBuf> {
    let dir_iter = if let Some(depth) = depth {
        WalkDir::new(root).max_depth(depth)
    } else {
        WalkDir::new(root)
    };
    dir_iter
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().into_path())
        .collect::<Vec<_>>()
}

pub fn read_files(root: &Path, depth: Option<usize>) -> Vec<PathBuf> {
    walkdir(root, depth)
        .into_iter()
        .filter(|path| path.is_file())
        .collect::<Vec<_>>()
}

pub fn read_dirs(root: &Path, depth: Option<usize>) -> Vec<PathBuf> {
    walkdir(root, depth)
        .into_iter()
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>()
}

pub fn filter_by_extension(files: Vec<PathBuf>, extensions: &Vec<String>) -> Vec<PathBuf> {
    if extensions.is_empty() {
        return files;
    }
    files
        .into_iter()
        .filter_map(|file| {
            extensions
                .contains(&file.extension()?.to_string_lossy().to_string())
                .then(|| file)
        })
        .collect::<Vec<_>>()
}

pub fn is_in_working_dir(working_dir: &PathBuf, file: &PathBuf) -> bool {
    match diff_paths(file, working_dir) {
        Some(diff) => diff.iter().count() != 1,
        None => false,
    }
}
