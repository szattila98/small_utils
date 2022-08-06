use cli::Args;
use logic::Rempref;
use std::env;
use structopt::StructOpt;

mod cli;
mod logic;

fn main() {
    let args = Args::from_args();
    let working_dir = env::current_dir().expect("failed to get working directory");
    let flush = args.do_renames;
    let mut rempref = Rempref::init(working_dir, args.into());

    let tasks = rempref.get_relativized_tasks();
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
        let (success_count, fail_count) = rempref.execute();

        if fail_count == 0 {
            println!("Renames successful, {success_count} files renamed!\n");
        } else if success_count == 0 {
            println!("All {fail_count} renames failed:");
            rempref
                .get_relativized_failed_tasks()
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
                .get_relativized_failed_tasks()
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
