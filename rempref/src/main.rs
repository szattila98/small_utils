use cli::Args;
use error::RemovePrefixError;
use glob::glob;
use std::{env, fs, path::PathBuf};
use structopt::StructOpt;
use task::RemovePrefixTask;

mod cli;
mod error;
mod task;

fn main() -> Result<(), RemovePrefixError> {
    let args = Args::from_args();
    let working_dir = get_working_dir()?;
    let files = read_files(format!("{}/*", working_dir))?;
    let tasks = create_tasks(args.prefix_length, files);
    show_changes(&tasks);
    if args.remove_prefix {
        remove_prefixes(tasks);
    }
    Ok(())
}

fn get_working_dir() -> Result<String, RemovePrefixError> {
    match env::current_dir() {
        Ok(path) => match path.into_os_string().into_string() {
            Ok(path) => Ok(path),
            Err(e) => Err(RemovePrefixError::WorkingDirParse(e)),
        },
        Err(e) => Err(RemovePrefixError::WorkingDirRetrieval(e)),
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

fn remove_prefixes(tasks: Vec<RemovePrefixTask>) {
    let mut failures = vec![];
    for task in &tasks {
        fs::rename(&task.from, &task.to).unwrap_or_else(|e| failures.push((task.clone(), e)));
    }
    if !failures.is_empty() {
        println!(
            "Modifications done but there are {} failed tasks:",
            failures.len()
        );
        failures
            .iter()
            .for_each(|(task, e)| println!("{} - {}", task.from, e));
    }
    println!(
        "Modifications done, removed the prefixes of {} files!",
        tasks.len() - failures.len()
    );
}
