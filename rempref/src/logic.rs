use std::{fs, io, path::PathBuf};

use commons::file::{
    errors::CheckBeforeError,
    functions::{filter_by_extension, read_files},
    model::FileOperationTask,
    traits::{CheckBefore, ExecuteTask, FileOperation, Instantiate, ToFileTask},
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
            let filename = &from.file_name().unwrap().to_string_lossy()[prefix_length.into()..];
            to.set_file_name(filename);
            FileOperationTask::new(from, to)
        });
    }
}

impl CheckBefore for Rempref {
    fn check_before(&self) -> Option<CheckBeforeError> {
        let mut overwritten = vec![];
        self.tasks.iter().for_each(|task| {
            self.tasks.iter().for_each(|other_task| {
                let is_clash = task.from != other_task.from && task.to == other_task.to;
                let overwrite_fail = task.to_failed("renaming this would overwrite another file");
                if task != other_task && is_clash && !overwritten.contains(&overwrite_fail) {
                    overwritten.push(overwrite_fail);
                }
            });
        });
        if !overwritten.is_empty() {
            overwritten.sort();
            Some(CheckBeforeError::FilesWouldOwerwrite(overwritten))
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
