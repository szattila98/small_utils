use commons::file::{
    errors::CheckBeforeError,
    is_hidden,
    model::FileOperationTask,
    traits::{ExecuteTask, FileOperation, Instantiate, ToFailed, ToFileTask},
    {filter_by_extension, read_files, walkdir},
};
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
            .into_iter()
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
        let mut would_overwrite = vec![];
        for task in self.tasks.iter() {
            for other_task in self.tasks.iter() {
                if task != other_task || self.tasks.len() == 1 {
                    let is_nested_clash = task.from != other_task.from && task.to == other_task.to;
                    let is_outer_clash = root_files.contains(&task.to);
                    let mut clashing_task_reason = vec![];
                    if is_nested_clash {
                        clashing_task_reason.push("would overwrite another renamed file");
                    }
                    if is_outer_clash {
                        clashing_task_reason.push("renaming would overwrite a file in root");
                    }
                    if !clashing_task_reason.is_empty() {
                        let fail = task.to_failed(
                            format!("\n- {}", &clashing_task_reason.join("\n- ")).as_str(),
                        );
                        would_overwrite.push(fail);
                    }
                }
            }
            let root_file = task
                .to
                .to_failed("would be overwritten in root by the rename of a file");
            if root_files.contains(&task.to) && !would_overwrite.contains(&root_file) {
                would_overwrite.push(root_file);
            }
        }
        if !would_overwrite.is_empty() {
            would_overwrite.sort();
            Some(CheckBeforeError::FilesWouldOverwrite(would_overwrite))
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
