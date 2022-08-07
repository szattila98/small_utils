use std::{fs, io, path::PathBuf};

use commons::file::{
    errors::FileOperationError,
    functions::{filter_by_extension, read_files},
    model::FileOperationTask,
    traits::{ExecuteTask, FileOperation, Instantiate, ScanForErrors, ToFileTask},
};

pub struct Config {
    prefix_length: u8,
    extensions: Vec<String>,
    recursive: bool,
}

impl Config {
    pub fn new(prefix_length: u8, extensions: Vec<String>, recursive: bool) -> Self {
        Self {
            prefix_length,
            extensions,
            recursive,
        }
    }
}

pub struct Rempref {
    tasks: Vec<FileOperationTask>,
    failed_tasks: Vec<(usize, io::Error)>,
}

impl Instantiate<Config> for Rempref {
    fn new(working_dir: PathBuf, config: Config) -> Self {
        let files = if config.recursive {
            read_files(&working_dir, None)
        } else {
            read_files(&working_dir, Some(1))
        };
        let filtered_files = filter_by_extension(files, &config.extensions);
        let mut rempref = Self {
            tasks: vec![],
            failed_tasks: vec![],
        };
        rempref.create_tasks(config.prefix_length, filtered_files);
        rempref
    }
}

impl Rempref {
    fn create_tasks(&mut self, prefix_length: u8, files: Vec<PathBuf>) {
        self.tasks = files.to_file_tasks(|from| {
            let mut to = from.clone();
            to.pop();
            let filename = &from.file_name().unwrap().to_string_lossy()[prefix_length.into()..];
            to.push(filename);
            FileOperationTask::new(from, to)
        });
    }
}

impl ScanForErrors for Rempref {
    fn scan_for_errors(&self) -> Option<FileOperationError> {
        let mut overwritten = vec![];
        self.tasks.iter().for_each(|task| {
            self.tasks.iter().for_each(|other_task| {
                if task.from != other_task.from && task.to == other_task.to {
                    overwritten.push(task.clone());
                }
            });
        });
        if !overwritten.is_empty() {
            Some(FileOperationError::FilesWouldOwerwrite(overwritten))
        } else {
            None
        }
    }
}

impl ExecuteTask for Rempref {
    fn execute_task(task: &FileOperationTask) -> io::Result<()> {
        fs::rename(&task.from, &task.to)
    }
}

impl FileOperation<Config> for Rempref {
    fn get_tasks(&self) -> Vec<FileOperationTask> {
        self.tasks.clone()
    }

    fn get_failed_tasks(&self) -> &Vec<(usize, io::Error)> {
        &self.failed_tasks
    }

    fn get_failed_tasks_mut(&mut self) -> &mut Vec<(usize, io::Error)> {
        &mut self.failed_tasks
    }
}
