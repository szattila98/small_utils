use structopt::StructOpt;

use super::{
    errors::FileOperationError,
    model::{FailedFileOperation, FileOperationResult, FileOperationTask},
};
use std::{
    env,
    path::{Path, PathBuf},
};

pub trait Relativizable {
    fn relativize(&self, working_dir: &Path) -> Self;
}

pub trait ToFileTask: IntoIterator + Sized {
    fn to_file_tasks<T>(self, task_generator: T) -> Vec<FileOperationTask>
    where
        T: Fn(<Self as IntoIterator>::Item) -> FileOperationTask,
    {
        self.into_iter()
            .map(task_generator)
            .collect::<Vec<FileOperationTask>>()
    }
}

impl ToFileTask for Vec<PathBuf> {}

pub trait Instantiable<T> {
    fn new(working_dir: PathBuf, args: T) -> Self;
}

pub trait ScanForErrors {
    fn scan_for_errors(&self) -> Option<FileOperationError>;
}

pub trait FileOperation {
    fn get_tasks(&self) -> Vec<FileOperationTask>;

    fn get_failed_tasks(&self) -> Vec<FailedFileOperation>;

    fn execute(&mut self) -> Result<FileOperationResult, FileOperationError>;
}

pub trait DoExec {
    fn do_exec(&self) -> bool;
}

pub trait Runnable<A, C, T>
where
    A: StructOpt + Into<C> + DoExec,
    T: Instantiable<C> + FileOperation,
{
    fn run(operation_name: &str) {
        let args = A::from_args();
        let working_dir = env::current_dir().expect("failed to get working directory");
        let flush = args.do_exec();
        let mut file_operation = T::new(working_dir.clone(), args.into());

        let tasks = file_operation.get_tasks().relativize(&working_dir);
        if tasks.is_empty() {
            println!("No files found to be {operation_name}d with these arguments!\n");
            return;
        }

        println!("\nFile {operation_name}s to be made:");
        tasks.iter().for_each(|task| {
            println!("{task}");
        });
        println!();

        if flush {
            println!("\nExecuting {operation_name}s...");
            let res = file_operation.execute();
            if let Err(e) = &res {
                println!("Failed to execute {operation_name}s:");
                match e {
                    FileOperationError::FilesWouldOwerwrite(files) => {
                        println!("{e}");
                        files.iter().for_each(|task| {
                            println!("{}", task.relativize(&working_dir));
                        });
                    }
                }
                return;
            }
            let FileOperationResult { successful, failed } = res.unwrap();

            if failed == 0 {
                println!("Execution successful, {successful} files {operation_name}d!\n");
            } else if successful == 0 {
                println!("All {failed} {operation_name}s failed:");
                file_operation
                    .get_failed_tasks()
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
                    .get_failed_tasks()
                    .relativize(&working_dir)
                    .iter()
                    .for_each(|failed_task| {
                        println!("{failed_task}");
                    });
            }
            println!()
        } else {
            println!("Run with -d flag to execute {operation_name}s\n");
        }
    }
}
