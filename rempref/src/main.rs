use cli::Args;
use glob::glob;
use std::{env, ffi::OsString, fs, io, path::PathBuf};
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
    #[error("File pattern is invalid: '{0}'")]
    GlobPatternError(#[from] glob::PatternError),
}

fn main() -> Result<(), RemovePrefixError> {
    let args = Args::from_args();
    let working_dir = get_working_dir()?;
    let files = read_files(format!("{}/*", working_dir))?;
    let tasks = create_tasks(args.prefix_length, files);
    show_changes(&tasks);
    // TODO refactor to RemovePrefixTask method
    let mut failures = vec![];
    if args.remove_prefix {
        for task in &tasks {
            fs::rename(&task.from, &task.to).unwrap_or_else(|e| failures.push((task.clone(), e)));
        }
        if !failures.is_empty() {
            println!("Modifications done but there are failed tasks:");
            failures
                .iter()
                .for_each(|(task, e)| println!("{} - {}", task.from, e));
        }
        println!(
            "Modifications done, removed the prefixes of {} files!",
            tasks.len() - failures.len()
        );
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

fn read_files(glob_pattern: String) -> Result<Vec<PathBuf>, RemovePrefixError> {
    let mut files = vec![];
    for entry in glob(&glob_pattern)? {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    files.push(path);
                }
            }
            Err(e) => println!("{e}"),
        }
    }
    Ok(files)
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
