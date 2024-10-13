use commons::file::{
    errors::CheckBeforeError,
    is_hidden,
    model::FileOperationTask,
    traits::{ExecuteTask, FileOperation, Instantiate, ToFailed, ToFileTask},
    {filter_by_extension, read_files, walkdir},
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
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
        let filtered_files = filter_by_extension(files, &config.extensions)
            .into_par_iter()
            .filter(|file| !is_hidden(file))
            .collect::<Vec<_>>();
        let mut rempref = Self {
            working_dir,
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

impl ExecuteTask for Rempref {
    fn check_before_execution(&self) -> Option<CheckBeforeError> {
        let root_files = walkdir(&self.working_dir, Some(1));

        let would_overwrite: Vec<_> = self
            .tasks
            .par_iter()
            .flat_map(|task| {
                let mut clashing_task_reason = vec![];

                let task_clashes: Vec<_> = self
                    .tasks
                    .par_iter()
                    .filter_map(|other_task| {
                        if task != other_task || self.tasks.len() == 1 {
                            let is_nested_clash =
                                task.from != other_task.from && task.to == other_task.to;
                            let is_outer_clash = root_files.contains(&task.to);
                            let mut reasons = vec![];
                            if is_nested_clash {
                                reasons.push("would overwrite another renamed file");
                            }
                            if is_outer_clash {
                                reasons.push("renaming would overwrite a file in root");
                            }
                            if !reasons.is_empty() {
                                let fail = task.to_failed(&format!("\n- {}", reasons.join("\n- ")));
                                return Some(fail);
                            }
                        }
                        None
                    })
                    .collect();

                clashing_task_reason.extend(task_clashes);

                // Check if the task conflicts with root files
                if root_files.contains(&task.to) {
                    let root_file =
                        task.to_failed("would be overwritten in root by the rename of a file");
                    if !clashing_task_reason.contains(&root_file) {
                        clashing_task_reason.push(root_file);
                    }
                }

                clashing_task_reason
            })
            .collect();

        if !would_overwrite.is_empty() {
            let mut would_overwrite_sorted = would_overwrite.clone();
            would_overwrite_sorted.sort();
            Some(CheckBeforeError::FilesWouldOverwrite(
                would_overwrite_sorted,
            ))
        } else {
            None
        }
    }

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
