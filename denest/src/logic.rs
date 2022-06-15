use pathdiff::diff_paths;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

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
    tasks: Vec<DenestTask>,
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
        self.tasks
            .iter()
            .map(|task| task.relativize(&self.working_dir))
            .collect()
    }

    pub fn get_failed_tasks(&self) -> Vec<FailedDenestTask> {
        let mut failed_tasks = vec![];
        for (i, error) in &self.failed_tasks {
            if let Some(task) = self.tasks.get(*i) {
                failed_tasks.push((task.clone(), error.to_string()).into());
            }
        }
        failed_tasks
    }

    pub fn get_relativized_failed_tasks(&self) -> Vec<FailedDenestTask> {
        self.get_failed_tasks()
            .iter()
            .map(|task| task.relativize(&self.working_dir))
            .collect()
    }
}

#[derive(Clone)]
pub struct DenestTask {
    // TODO no publics
    pub from: PathBuf,
    pub to: PathBuf,
}

impl DenestTask {
    pub fn relativize(&self, working_dir: &PathBuf) -> Self {
        let mut task = self.clone();
        task.from = diff_paths(&task.from, working_dir).unwrap();
        task.to = diff_paths(&task.to, working_dir).unwrap();
        task
    }
}

impl From<(u8, PathBuf)> for DenestTask {
    fn from((prefix_len, from): (u8, PathBuf)) -> Self {
        let mut to = from.clone();
        to.pop();
        let filename = &from.file_name().unwrap().to_string_lossy()[prefix_len.into()..];
        to.push(filename);
        DenestTask { from, to }
    }
}

#[derive(Clone)]
pub struct FailedDenestTask {
    pub file_path: PathBuf,
    pub reason: String,
}

impl FailedDenestTask {
    pub fn relativize(&self, working_dir: &PathBuf) -> Self {
        let mut task = self.clone();
        task.file_path = diff_paths(task.file_path, working_dir).unwrap();
        task
    }
}

impl From<(DenestTask, String)> for FailedDenestTask {
    fn from((task, reason): (DenestTask, String)) -> Self {
        FailedDenestTask {
            file_path: task.from,
            reason,
        }
    }
}
