use cli::Args;
use commons::FileOperationError;
use logic::Denest;
use std::env;
use structopt::StructOpt;

mod cli;
mod logic;

fn main() {
    let args = Args::from_args();
    let working_dir = env::current_dir().expect("failed to get working directory");
    let flush = args.do_moves;
    let mut denest = Denest::init(working_dir.clone(), args.into());

    let tasks = denest.get_relativized_tasks();
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
        let (success_count, fail_count) = res.unwrap();

        if fail_count == 0 {
            println!("Moves successful, {success_count} files moved!\n");
        } else if success_count == 0 {
            println!("All {fail_count} moved failed:");
            denest
                .get_relativized_failed_tasks()
                .iter()
                .for_each(|failed_task| {
                    println!("{failed_task}");
                });
        } else {
            println!(
                "{} moves are successful, but {} moves failed:",
                success_count, fail_count
            );
            denest
                .get_relativized_failed_tasks()
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
