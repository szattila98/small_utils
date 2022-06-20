use pathdiff::diff_paths;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

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
        let files = Self::read_files(&working_dir, &config);
        let fileter_files = Self::filter_files(files, &config);
        let tasks = Self::create_tasks(config.prefix_length, fileter_files);
        Self {
            working_dir,
            tasks,
            failed_tasks: vec![],
        }
    }

    fn read_files(working_dir: &Path, config: &Config) -> Vec<PathBuf> {
        let dir_iter = if !config.recursive {
            WalkDir::new(working_dir).max_depth(1)
        } else {
            WalkDir::new(working_dir)
        };
        let mut files = vec![];
        for result in dir_iter {
            match result {
                Ok(entry) => {
                    let path = entry.into_path();
                    if path.is_file() && !path.is_symlink() {
                        files.push(path);
                    }
                }
                Err(e) => println!("{e}"),
            }
        }
        files
    }

    fn filter_files(files: Vec<PathBuf>, config: &Config) -> Vec<PathBuf> {
        if !config.extensions.is_empty() {
            let mut filtered_files = vec![];
            for file in files {
                let file_extension = match file.extension() {
                    Some(ext) => ext.to_string_lossy().to_string(),
                    None => continue,
                };
                if config.extensions.contains(&file_extension) {
                    filtered_files.push(file);
                }
            }
            return filtered_files;
        }
        files
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
