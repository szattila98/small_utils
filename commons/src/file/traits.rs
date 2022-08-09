#![allow(clippy::result_unit_err)]

use super::{
    errors::CheckBeforeError,
    model::{FailedFileOperation, FileOperationResult, FileOperationTask},
};
use std::{
    env, io,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

pub trait Relativize {
    fn relativize(&self, working_dir: &Path) -> Self;
}

pub trait ToFileTask: IntoIterator + Sized {
    fn to_file_tasks<T>(self, task_generator: T) -> Vec<FileOperationTask>
    where
        T: Fn(<Self as IntoIterator>::Item) -> FileOperationTask,
    {
        let mut tasks = self
            .into_iter()
            .map(task_generator)
            .collect::<Vec<FileOperationTask>>();
        tasks.sort();
        tasks
    }
}

impl ToFileTask for Vec<PathBuf> {}

pub trait ToFailed {
    fn to_failed(&self, reason: &str) -> FailedFileOperation;
}

pub trait Instantiate<C> {
    fn new(working_dir: PathBuf, args: C) -> Self;
}

pub trait ExecuteTask {
    fn check_before_execution(&self) -> Option<CheckBeforeError> {
        None
    }

    fn before_execute(&self) -> Result<bool, ()> {
        Ok(false)
    }

    fn execute_task(task: &FileOperationTask) -> io::Result<()>;

    fn after_execute(&self) -> Result<bool, ()> {
        Ok(false)
    }
}

pub trait FileOperation<C>: Instantiate<C> + ExecuteTask {
    fn get_tasks(&self) -> Vec<FileOperationTask>;

    fn get_failed_tasks(&self) -> &Vec<(usize, io::Error)>;

    fn get_failed_tasks_mut(&mut self) -> &mut Vec<(usize, io::Error)>;

    fn get_failed_operations(&self) -> Vec<FailedFileOperation> {
        let mut failed_tasks = vec![];
        for (i, error) in self.get_failed_tasks() {
            if let Some(task) = self.get_tasks().get(*i) {
                failed_tasks.push(FailedFileOperation::new(
                    task.from.clone(),
                    error.to_string(),
                ));
            }
        }
        failed_tasks
    }

    fn execute(&mut self) -> FileOperationResult {
        self.get_tasks().iter().enumerate().for_each(|(i, task)| {
            if let Err(e) = Self::execute_task(task) {
                self.get_failed_tasks_mut().push((i, e))
            }
        });
        FileOperationResult::new(
            self.get_tasks().len() - self.get_failed_tasks().len(),
            self.get_failed_tasks().len(),
        )
    }
}

pub trait InputArgs {
    fn working_dir(&self) -> Option<PathBuf>;

    fn do_exec(&self) -> bool;
}

pub trait Runnable<A, C, T>
where
    A: StructOpt + Into<C> + InputArgs,
    T: Instantiate<C> + FileOperation<C>,
{
    fn name() -> String;

    fn verb() -> String;

    fn run() {
        let args = A::from_args();
        let working_dir = match args.working_dir() {
            Some(dir) => dir,
            None => env::current_dir().expect("failed to get working directory"),
        };
        let flush = args.do_exec();
        let mut file_operation = T::new(working_dir.clone(), args.into());

        println!("{}", Self::name());
        let operation_name = Self::verb();

        let tasks = file_operation.get_tasks().relativize(&working_dir);
        if tasks.is_empty() {
            println!("\nNo files found to be {operation_name}d with these arguments!\n");
            return;
        }

        println!("\nFile {operation_name}s to be made:");
        tasks.iter().for_each(|task| {
            println!("{task}");
        });

        println!("\nRunning checks before execution...");
        if let Some(e) = file_operation.check_before_execution() {
            println!("Failed to execute {operation_name}s:");
            match &e {
                CheckBeforeError::FilesWouldOwerwrite(files) => {
                    println!("{e}\n");
                    files.iter().for_each(|task| {
                        println!("{}", task.relativize(&working_dir));
                    });
                }
            }
            return;
        } else {
            println!("All checks passed!");
        }

        if flush {
            println!("\nBefore execution running...");
            match file_operation.before_execute() {
                Ok(ran) => {
                    if ran {
                        println!("Before execution ran successfully!");
                    } else {
                        println!("No before exectution ran!");
                    }
                }
                Err(_) => {
                    println!("Before execution failed!");
                    return;
                }
            };

            println!("\nExecuting {operation_name}s...");
            let FileOperationResult { successful, failed } = file_operation.execute();
            if failed == 0 {
                println!("Execution successful, {successful} files {operation_name}d!");
            } else if successful == 0 {
                println!("All {failed} {operation_name}s failed:");
                file_operation
                    .get_failed_operations()
                    .relativize(&working_dir)
                    .iter()
                    .for_each(|failed_task| {
                        println!("{failed_task}");
                    });
            } else {
                println!(
                    "{successful} {operation_name}s are successful, but {failed} {operation_name}s failed:"
                );
                file_operation
                    .get_failed_operations()
                    .relativize(&working_dir)
                    .iter()
                    .for_each(|failed_task| {
                        println!("{failed_task}");
                    });
            }

            println!("\nAfter execution running...");
            match file_operation.after_execute() {
                Ok(ran) => {
                    if ran {
                        println!("After execution ran successfully!");
                    } else {
                        println!("No after exectution ran!");
                    }
                }
                Err(_) => {
                    println!("After execution failed!");
                }
            };
        } else {
            println!("Run with -d flag to execute {operation_name}s\n");
        }
    }
}
