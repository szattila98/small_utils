use crate::error::RemPrefError;
use std::{env, fs, io, path::PathBuf};
use wax::Glob;

pub struct Rempref {
    tasks: Vec<RemPrefTask>,
    successful_tasks: Vec<usize>,
    failed_tasks: Vec<(usize, io::Error)>,
}

impl Rempref {
    pub fn init(config: Config) -> Result<Self, RemPrefError> {
        let working_dir = Self::get_working_dir()?;
        let glob_pattern = Self::generate_glob_pattern(&config);
        let files = Self::read_files(working_dir, glob_pattern)?;
        let tasks = Self::create_tasks(config.prefix_length, files);
        Ok(Self {
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

    pub fn get_failed_tasks(&self) -> Vec<FailedRemprefTask> {
        let mut failed_tasks = vec![];
        for (i, task) in self.tasks.iter().enumerate() {
            match self.failed_tasks.get(i) {
                Some((_, e)) => failed_tasks.push((task.clone(), e.to_string()).into()),
                None => (),
            }
        }
        failed_tasks
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

    fn generate_glob_pattern(config: &Config) -> String {
        let base = "/*".to_string(); // TODO builder for pattern maybe?
        if config.extensions.is_empty() {
            base
        } else {
            println!("{}.{{{}}}", base, config.extensions.join(","));
            format!("{}.{{{}}}", base, config.extensions.join(","))
        }
    }

    fn read_files(working_dir: String, glob_pattern: String) -> Result<Vec<PathBuf>, RemPrefError> {
        let exp = format!("{working_dir}{glob_pattern}");
        let glob = Glob::new(&exp)?;
        let mut files = vec![];
        for entry in glob.walk("/") {
            match entry {
                Ok(res) => {
                    let path = res.into_path();
                    if path.is_file() && !path.is_symlink() {
                        files.push(path);
                    }
                }
                Err(e) => println!("{e}"), // TODO better errorhandling
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

impl From<(u8, PathBuf)> for RemPrefTask {
    fn from((prefix_len, from): (u8, PathBuf)) -> Self {
        let mut to = from.clone();
        to.pop();
        let filename = &from.file_name().unwrap().to_string_lossy()[prefix_len.into()..];
        to.push(filename);
        RemPrefTask { from, to }
    }
}

pub struct FailedRemprefTask {
    pub file_path: PathBuf,
    pub reason: String,
}

impl From<(RemPrefTask, String)> for FailedRemprefTask {
    fn from((task, reason): (RemPrefTask, String)) -> Self {
        FailedRemprefTask {
            file_path: task.from,
            reason,
        }
    }
}