use cli::Args;
use commons::file::{
    errors::FileOperationError,
    model::FileOperationResult,
    traits::{FileOperation, Relativizable},
};
use logic::Denest;
use std::env;
use structopt::StructOpt;

mod cli;
mod logic;

fn main() {
    let args = Args::from_args();
    let working_dir = env::current_dir().expect("failed to get working directory");
    let flush = args.do_moves;
    let mut denest = Denest::new(working_dir.clone(), args.into());

    let tasks = denest.get_tasks().relativize(&working_dir);
    if tasks.is_empty() {
        println!("No files found to be moved with these arguments!\n");
        return;
    }

    println!("\nMoves to be made:");
    tasks.iter().for_each(|task| {
        println!("{task}");
    });
    println!();

    if flush {
        println!("\nExecuting moves...");
        let res = denest.execute();
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
        let FileOperationResult { successful, failed } = res.unwrap();

        if failed == 0 {
            println!("Moves successful, {successful} files moved!\n");
        } else if successful == 0 {
            println!("All {failed} moved failed:");
            denest
                .get_failed_tasks()
                .relativize(&working_dir)
                .iter()
                .for_each(|failed_task| {
                    println!("{failed_task}");
                });
        } else {
            println!(
                "{} moves are successful, but {} moves failed:",
                successful, failed
            );
            denest
                .get_tasks()
                .relativize(&working_dir)
                .iter()
                .for_each(|failed_task| {
                    println!("{failed_task}");
                });
        }
        println!()
    } else {
        println!("Run with -d flag to execute moved\n");
    }
}
