use commons::{filter_by_extension, read_files, FailedFileOperation, FileOperationTask};
use std::{fs, io, path::PathBuf};
pub struct Config {
    extensions: Vec<String>,
    depth: Option<u8>,
}

impl Config {
    pub fn new(extensions: Vec<String>, depth: Option<u8>) -> Self {
        Self { extensions, depth }
    }
}

pub struct Denest {
    working_dir: PathBuf,
    tasks: Vec<FileOperationTask>,
    failed_tasks: Vec<(usize, io::Error)>,
}

impl Denest {
    pub fn init(working_dir: PathBuf, config: Config) -> Self {
        todo!()
    }

    fn read_files(working_dir: &Path, config: &Config) -> Vec<PathBuf> {
        todo!()
    }

    fn filter_files(files: Vec<PathBuf>, config: &Config) -> Vec<PathBuf> {
        todo!()
    }

    fn create_tasks(prefix_length: u8, files: Vec<PathBuf>) -> Vec<DenestTask> {
        todo!()
    }

    pub fn execute(&mut self) -> (usize, usize) {
        todo!()
    }

    pub fn get_relativized_tasks(&self) -> Vec<DenestTask> {
        todo!()
    }

    pub fn get_failed_tasks(&self) -> Vec<FailedDenestTask> {
        todo!()
    }

    pub fn get_relativized_failed_tasks(&self) -> Vec<FailedDenestTask> {
        todo!()
    }
}
