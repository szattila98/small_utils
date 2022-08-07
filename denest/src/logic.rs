use std::{fs, io, path::PathBuf};

use commons::file::{
    errors::FileOperationError,
    functions::{filter_by_extension, read_files},
    model::{FailedFileOperation, FileOperationResult, FileOperationTask},
    traits::{FileOperation, Instantiable, ToFileTask},
};
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

impl Instantiable<Config> for Denest {
    fn new(working_dir: PathBuf, config: Config) -> Self {
        let files = if let Some(depth) = config.depth {
            read_files(&working_dir, Some(depth.into()))
        } else {
            read_files(&working_dir, None)
        };
        let filtered_files = filter_by_extension(files, &config.extensions);
        let mut denest = Self {
            working_dir,
            tasks: vec![],
            failed_tasks: vec![],
        };
        denest.create_tasks(filtered_files);
        denest
    }
}

impl Denest {
    fn create_tasks(&mut self, files: Vec<PathBuf>) {
        self.tasks = files.to_file_tasks(|from| {
            let mut to = self.working_dir.clone();
            let filename = &from.file_name().unwrap().to_string_lossy().to_string();
            to.push(filename);
            FileOperationTask::new(from, to)
        })
    }
}

impl FileOperation for Denest {
    fn get_tasks(&self) -> Vec<FileOperationTask> {
        self.tasks.clone()
    }

    fn get_failed_tasks(&self) -> Vec<FailedFileOperation> {
        let mut failed_tasks = vec![];
        for (i, error) in &self.failed_tasks {
            if let Some(task) = self.tasks.get(*i) {
                failed_tasks.push(FailedFileOperation::new(
                    task.from.clone(),
                    error.to_string(),
                ));
            }
        }
        failed_tasks
    }

    fn execute(&mut self) -> Result<FileOperationResult, FileOperationError> {
        let mut overwritten = vec![];
        self.tasks.iter().for_each(|task| {
            self.tasks.iter().for_each(|other_task| {
                if task.from != other_task.from && task.to == other_task.to {
                    overwritten.push(task.clone());
                }
            });
        });
        if !overwritten.is_empty() {
            return Err(FileOperationError::FilesWouldOwerwrite(overwritten));
        }

        self.tasks.iter().enumerate().for_each(|(i, task)| {
            if let Err(e) = fs::rename(&task.from, &task.to) {
                self.failed_tasks.push((i, e))
            }
        });
        Ok(FileOperationResult::new(
            self.tasks.len() - self.failed_tasks.len(),
            self.failed_tasks.len(),
        ))
    }
}
