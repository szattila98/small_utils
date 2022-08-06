use commons::{
    filter_by_extension, is_in_working_dir, read_files, FailedFileOperation, FileOperationTask,
};
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
        let files = if let Some(depth) = config.depth {
            read_files(&working_dir, Some(depth.into()))
        } else {
            read_files(&working_dir, None)
        };
        let filtered_files = filter_by_extension(files, &config.extensions)
            .into_iter()
            .filter(|file| is_in_working_dir(&working_dir, file))
            .collect::<Vec<_>>();
        let mut denest = Self {
            working_dir,
            tasks: vec![],
            failed_tasks: vec![],
        };
        denest.create_tasks(filtered_files);
        denest
    }

    fn create_tasks(&mut self, files: Vec<PathBuf>) {
        self.tasks = files
            .iter()
            .map(|from| {
                let from = from.clone();
                let mut to = self.working_dir.clone();
                let filename = &from.file_name().unwrap().to_string_lossy().to_string();
                to.push(filename);
                FileOperationTask::new(from, to)
            })
            .collect::<Vec<FileOperationTask>>();
    }

    pub fn execute(&mut self) -> (usize, usize) {
        self.tasks.iter().enumerate().for_each(|(i, task)| {
            if let Err(e) = fs::rename(&task.from, &task.to) {
                self.failed_tasks.push((i, e))
            }
        });
        (
            self.tasks.len() - self.failed_tasks.len(),
            self.failed_tasks.len(),
        )
    }

    pub fn get_relativized_tasks(&self) -> Vec<FileOperationTask> {
        self.tasks
            .iter()
            .map(|task| task.relativize(&self.working_dir))
            .collect()
    }

    pub fn get_failed_tasks(&self) -> Vec<FailedFileOperation> {
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

    pub fn get_relativized_failed_tasks(&self) -> Vec<FailedFileOperation> {
        self.get_failed_tasks()
            .iter()
            .map(|task| task.relativize(&self.working_dir))
            .collect()
    }
}
