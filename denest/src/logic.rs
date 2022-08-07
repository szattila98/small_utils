use std::{fs, io, path::PathBuf};

use commons::file::{
    errors::FileOperationError,
    functions::{filter_by_extension, read_dirs, read_files},
    model::FileOperationTask,
    traits::{ExecuteTask, FileOperation, Instantiate, ScanForErrors, ToFileTask},
};
pub struct Config {
    extensions: Vec<String>,
    depth: Option<u8>,
    cleanup: bool,
}

impl Config {
    pub fn new(extensions: Vec<String>, depth: Option<u8>, cleanup: bool) -> Self {
        Self {
            extensions,
            depth,
            cleanup,
        }
    }
}

pub struct Denest {
    working_dir: PathBuf,
    tasks: Vec<FileOperationTask>,
    failed_tasks: Vec<(usize, io::Error)>,
    cleanup: bool,
}

impl Instantiate<Config> for Denest {
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
            cleanup: config.cleanup,
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
        });
    }
}

impl ScanForErrors for Denest {
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

impl ExecuteTask for Denest {
    fn execute_task(task: &FileOperationTask) -> io::Result<()> {
        fs::rename(&task.from, &task.to)
    }

    fn finally(&self) {
        let mut dirs = read_dirs(&self.working_dir, None);
        dirs.reverse();
        println!("{dirs:?}");
        for dir in dirs {
            let is_empty = match dir.read_dir() {
                Ok(mut reader) => reader.next().is_none(),
                Err(_) => false,
            };
            if self.cleanup && is_empty {
                fs::remove_dir(dir);
            }
        }
    }
}

impl FileOperation<Config> for Denest {
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
