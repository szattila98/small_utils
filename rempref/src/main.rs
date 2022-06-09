use cli::Args;
use glob::glob;
use std::{fs, path::PathBuf};
use structopt::StructOpt;
use task::RemovePrefixTask;

mod cli;
mod task;

fn main() {
    let args = Args::from_args();
    let working_dir = get_working_dir();
    let files = read_files(format!("{}/*", working_dir));
    let tasks = files
        .iter()
        .map(|file| (args.prefix_len, file.display().to_string()).into())
        .collect::<Vec<RemovePrefixTask>>();
    for task in &tasks {
        println!("{}", task);
    }
    if args.modify {
        for task in tasks {
            fs::rename(task.from, task.to).expect("Rename failed!");
        }
    }
}

fn get_working_dir() -> String {
    std::env::current_dir()
        .expect("Permission denied or current directory does not exist")
        .into_os_string()
        .into_string()
        .expect("Current directory string is invalid")
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
