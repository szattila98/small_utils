use cli::Args;
use glob::glob;
use std::{
    env,
    ffi::OsString,
    fs,
    io::{self, Error},
    path::PathBuf,
};
use structopt::StructOpt;
use task::RemovePrefixTask;
use thiserror::Error;

mod cli;
mod task;

// TODO result error handling everywhere
#[derive(Debug, Error)]
pub enum RemovePrefixError {
    #[error("Could not get working dir, reason: '{0}'")]
    WorkingDirRetrievalError(io::Error),
    #[error("Current directory string is invalid: '{0:?}'")]
    WorkingDirParseError(OsString),

    #[error("Rename operation failed, reason: '{0}'")]
    RenameError(#[from] io::Error),
}

fn main() -> Result<(), RemovePrefixError> {
    let args = Args::from_args();
    let working_dir = get_working_dir()?;
    let files = read_files(format!("{}/*", working_dir));
    let tasks = create_tasks(args.prefix_length, files);
    show_changes(&tasks);
    if args.remove_prefix {
        for task in tasks {
            // TODO refactor to RemovePrefixTask method
            // TODO if some fails, there should be a list which contains the failed tasks and the reasons
            fs::rename(task.from, task.to).expect("Rename failed!");
        }
        println!("Modifications done!");
    }
    Ok(())
}

fn get_working_dir() -> Result<String, RemovePrefixError> {
    match env::current_dir() {
        Ok(path) => match path.into_os_string().into_string() {
            Ok(path) => Ok(path),
            Err(e) => Err(RemovePrefixError::WorkingDirParseError(e)),
        },
        Err(e) => Err(RemovePrefixError::WorkingDirRetrievalError(e)),
    }
}

fn read_files(glob_pattern: String) -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    files.push(path);
                }
            }
            Err(e) => println!("{e}"),
        }
    }
    files
}

fn create_tasks(prefix_length: u8, files: Vec<PathBuf>) -> Vec<RemovePrefixTask> {
    files
        .iter()
        .map(|file| (prefix_length, file.display().to_string()).into())
        .collect::<Vec<RemovePrefixTask>>()
}

fn show_changes(tasks: &Vec<RemovePrefixTask>) {
    println!("Changes:");
    for task in tasks {
        println!("{}", task);
    }
}
