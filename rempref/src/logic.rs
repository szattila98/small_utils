use crate::error::RemPrefError;
use glob::glob;
use std::{
    env, fs, io,
    path::{PathBuf, MAIN_SEPARATOR},
};

pub struct Rempref {
    // config: Config,
    tasks: Vec<RemPrefTask>,
    successful_tasks: Vec<usize>,
    failed_tasks: Vec<(usize, io::Error)>,
}

impl Rempref {
    pub fn init(config: Config) -> Result<Self, RemPrefError> {
        let working_dir = Self::get_working_dir()?;
        let files = Self::read_files(format!("{}/*", working_dir))?;
        let tasks = Self::create_tasks(config.prefix_length, files);
        Ok(Self {
            // config,
            tasks,
            successful_tasks: vec![],
            failed_tasks: vec![],
        })
    }

    pub fn get_tasks(&self) -> &Vec<RemPrefTask> {
        &self.tasks
    }

    // pub fn get_successful_tasks(&self) -> Vec<RemPrefTask> {
    //     self.tasks
    //         .iter()
    //         .enumerate()
    //         .filter(|(i, _)| self.successful_tasks.contains(i))
    //         .map(|(_, t)| t.clone())
    //         .collect()
    // }

    pub fn get_failed_tasks(&self) -> Vec<RemPrefTask> {
        self.tasks
            .iter()
            .enumerate()
            .filter(|(i, _)| self.failed_tasks.iter().any(|(j, _)| j == i))
            .map(|(_, t)| t.clone())
            .collect()
    }

    pub fn execute(&mut self) -> (usize, usize) {
        self.tasks.iter().enumerate().for_each(|(i, task)| {
            match fs::rename(&task.from, &task.to) {
                Ok(_) => self.successful_tasks.push(i),
                Err(e) => self.failed_tasks.push((i, e)),
            }
        });
        (self.successful_tasks.len(), self.failed_tasks.len())
    }

    fn get_working_dir() -> Result<String, RemPrefError> {
        match env::current_dir() {
            Ok(path) => match path.into_os_string().into_string() {
                Ok(path) => Ok(path),
                Err(e) => Err(RemPrefError::WorkingDirParseFailed(e)),
            },
            Err(e) => Err(RemPrefError::WorkingDirRetrievalFailed(e)),
        }
    }

    fn read_files(glob_pattern: String) -> Result<Vec<PathBuf>, RemPrefError> {
        let mut files = vec![];
        for entry in glob(&glob_pattern)? {
            match entry {
                Ok(path) => {
                    if path.is_file() && !path.is_symlink() {
                        files.push(path);
                    }
                }
                Err(e) => println!("{e}"),
            }
        }
        Ok(files)
    }

    fn create_tasks(prefix_length: u8, files: Vec<PathBuf>) -> Vec<RemPrefTask> {
        files
            .iter()
            .map(|file| (prefix_length, file.display().to_string()).into())
            .collect::<Vec<RemPrefTask>>()
    }
}

pub struct Config {
    pub prefix_length: u8,
}

#[derive(Clone)]
pub struct RemPrefTask {
    pub from: String,
    pub to: String,
}

impl From<(u8, String)> for RemPrefTask {
    fn from((prefix_len, from): (u8, String)) -> Self {
        let mut path = from.split(MAIN_SEPARATOR).collect::<Vec<_>>();
        let last_part = &path.remove(path.len() - 1)[prefix_len.into()..];
        path.push(last_part);
        let to = path.join(MAIN_SEPARATOR.to_string().as_str());
        RemPrefTask { from, to }
    }
}
