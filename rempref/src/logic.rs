use commons::{diff_paths, filter_by_extension, read_files};
use std::{fs, io, path::PathBuf};

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
    working_dir: PathBuf,
    tasks: Vec<RemPrefTask>,
    failed_tasks: Vec<(usize, io::Error)>,
}

impl Rempref {
    pub fn init(working_dir: PathBuf, config: Config) -> Self {
        let files = if config.recursive {
            read_files(&working_dir, None)
        } else {
            read_files(&working_dir, Some(1))
        };
        let filter_files = filter_by_extension(files, &config.extensions);
        let tasks = Self::create_tasks(config.prefix_length, filter_files);
        Self {
            working_dir,
            tasks,
            failed_tasks: vec![],
        }
    }

    fn create_tasks(prefix_length: u8, files: Vec<PathBuf>) -> Vec<RemPrefTask> {
        files
            .iter()
            .map(|file| (prefix_length, file.to_owned()).into())
            .collect::<Vec<RemPrefTask>>()
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

    pub fn get_relativized_tasks(&self) -> Vec<RemPrefTask> {
        self.tasks
            .iter()
            .map(|task| task.relativize(&self.working_dir))
            .collect()
    }

    pub fn get_failed_tasks(&self) -> Vec<FailedRemprefTask> {
        let mut failed_tasks = vec![];
        for (i, error) in &self.failed_tasks {
            if let Some(task) = self.tasks.get(*i) {
                failed_tasks.push((task.clone(), error.to_string()).into());
            }
        }
        failed_tasks
    }

    pub fn get_relativized_failed_tasks(&self) -> Vec<FailedRemprefTask> {
        self.get_failed_tasks()
            .iter()
            .map(|task| task.relativize(&self.working_dir))
            .collect()
    }
}

#[derive(Clone)]
pub struct RemPrefTask {
    // TODO no publics
    pub from: PathBuf,
    pub to: PathBuf,
}

impl RemPrefTask {
    pub fn relativize(&self, working_dir: &PathBuf) -> Self {
        let mut task = self.clone();
        task.from = diff_paths(&task.from, working_dir).unwrap();
        task.to = diff_paths(&task.to, working_dir).unwrap();
        task
    }
}

impl From<(u8, PathBuf)> for RemPrefTask {
    fn from((prefix_len, from): (u8, PathBuf)) -> Self {
        let mut to = from.clone();
        to.pop();
        let filename = &from.file_name().unwrap().to_string_lossy()[prefix_len.into()..];
        to.push(filename);
        RemPrefTask { from, to }
    }
}

#[derive(Clone)]
pub struct FailedRemprefTask {
    pub file_path: PathBuf,
    pub reason: String,
}

impl FailedRemprefTask {
    pub fn relativize(&self, working_dir: &PathBuf) -> Self {
        let mut task = self.clone();
        task.file_path = diff_paths(task.file_path, working_dir).unwrap();
        task
    }
}

impl From<(RemPrefTask, String)> for FailedRemprefTask {
    fn from((task, reason): (RemPrefTask, String)) -> Self {
        FailedRemprefTask {
            file_path: task.from,
            reason,
        }
    }
}
