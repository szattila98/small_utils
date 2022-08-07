use cli::Args;
use commons::{error::FileOperationError, Relativizable};
use logic::Rempref;
use std::env;
use structopt::StructOpt;

mod cli;
mod logic;

fn main() {
    let args = Args::from_args();
    let working_dir = env::current_dir().expect("failed to get working directory");
    let flush = args.do_renames;
    let mut rempref = Rempref::init(working_dir.clone(), args.into());

    let tasks = rempref.get_tasks().relativize(&working_dir);
    if tasks.is_empty() {
        println!("No files found to be renamed with these arguments!\n");
        return;
    }

    println!("\nRenames to be made:");
    tasks.iter().for_each(|task| {
        println!("{task}");
    });
    println!();

    if flush {
        println!("\nExecuting renames...");
        let res = rempref.execute();
        if let Err(e) = &res {
            println!("Failed to execute:");
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
        let (success_count, fail_count) = res.unwrap();

        if fail_count == 0 {
            println!("Renames successful, {success_count} files renamed!\n");
        } else if success_count == 0 {
            println!("All {fail_count} renames failed:");
            rempref
                .get_failed_tasks()
                .relativize(&working_dir)
                .iter()
                .for_each(|failed_task| {
                    println!("{failed_task}");
                });
        } else {
            println!(
                "{} renames are successful, but {} renames failed:",
                success_count, fail_count
            );
            rempref
                .get_failed_tasks()
                .relativize(&working_dir)
                .iter()
                .for_each(|failed_task| {
                    println!("{failed_task}");
                });
        }
        println!()
    } else {
        println!("Run with -d flag to execute renames\n");
    }
}
