use std::{fs, io, path::PathBuf};

use commons::file::{
    errors::CheckBeforeError,
    functions::{filter_by_extension, is_in_working_dir, read_dirs, read_files},
    model::FileOperationTask,
    traits::{CheckBefore, ExecuteTask, FileOperation, Instantiate, ToFailed, ToFileTask},
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
        let filtered_files = filter_by_extension(files, &config.extensions)
            .into_iter()
            .filter(|file| is_in_working_dir(&working_dir, file))
            .collect::<Vec<_>>();
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

impl CheckBefore for Denest {
    fn check_before(&self) -> Option<CheckBeforeError> {
        let root_files = read_files(&self.working_dir, Some(1));
        let mut would_overwrite = vec![];
        for task in self.tasks.iter() {
            for other_task in self.tasks.iter() {
                if task != other_task || self.tasks.len() == 1 {
                    let is_nested_clash = task.from != other_task.from && task.to == other_task.to;
                    let is_outer_clash = root_files.contains(&task.to);
                    let mut clashing_task_reason = vec![];
                    if dbg!(is_nested_clash) {
                        clashing_task_reason.push("would overwrite another moved file");
                    }
                    if dbg!(is_outer_clash) {
                        clashing_task_reason.push("would overwrite a file in root");
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
                .to_failed("would be overwritten in root by the move of a nested file");
            if root_files.contains(&task.to) && !would_overwrite.contains(&root_file) {
                would_overwrite.push(root_file);
            }
        }
        if !would_overwrite.is_empty() {
            would_overwrite.sort();
            Some(CheckBeforeError::FilesWouldOwerwrite(would_overwrite))
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
        if self.cleanup {
            let mut dirs = read_dirs(&self.working_dir, None);
            dirs.reverse();
            dirs.into_iter().for_each(|dir| {
                let _ = fs::remove_dir(dir);
            });
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
