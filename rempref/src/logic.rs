use crate::error::RemPrefError;
use nu_glob::glob;
use pathdiff::diff_paths;
use std::{
    fmt::Display,
    fs, io,
    path::{self, Path, PathBuf},
};

pub struct Rempref {
    working_dir: PathBuf,
    tasks: Vec<RemPrefTask>,
    failed_tasks: Vec<(usize, io::Error)>,
}

impl Rempref {
    pub fn init(working_dir: PathBuf, config: Config) -> Result<Self, RemPrefError> {
        let glob_pattern = Self::generate_glob_pattern(&config);
        let files = Self::read_files(&working_dir, glob_pattern)?;
        let tasks = Self::create_tasks(config.prefix_length, files);
        Ok(Self {
            working_dir,
            tasks,
            failed_tasks: vec![],
        })
    }

    fn generate_glob_pattern(config: &Config) -> String {
        let base = format!("{}*", path::MAIN_SEPARATOR); // TODO builder for pattern maybe?
        if config.extensions.is_empty() {
            base
        } else {
            format!("{}?({})", base, config.extensions.join("|.")) // TODO make this work
        }
    }

    fn read_files(working_dir: &Path, glob_pattern: String) -> Result<Vec<PathBuf>, RemPrefError> {
        let exp = format!("{}{}", working_dir.display(), glob_pattern);
        println!("{}", exp);
        let mut files = vec![];
        for entry in glob(&exp)? {
            match entry {
                Ok(path) => {
                    if path.is_file() && !path.is_symlink() {
                        files.push(path);
                    }
                }
                Err(e) => println!("{e}"), // TODO better error handling
            }
        }
        Ok(files)
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

    // pub fn get_tasks(&self) -> Vec<RemPrefTask> {
    //     self.tasks.clone()
    // }

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

pub struct Config {
    pub prefix_length: u8,
    pub extensions: Vec<String>,
}

#[derive(Clone)]
pub struct RemPrefTask {
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

impl Display for RemPrefTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from.display(), self.to.display())
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

impl Display for FailedRemprefTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.file_path.display(), self.reason)
    }
}
