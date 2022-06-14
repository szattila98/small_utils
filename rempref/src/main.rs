use cli::Args;
use error::RemPrefError;
use logic::Rempref;
use structopt::StructOpt;

mod cli;
mod error;
mod logic;

fn main() -> Result<(), RemPrefError> {
    let args = Args::from_args();
    let flush = args.do_renames;
    let mut rempref = Rempref::init(args.into())?;
    println!("\nRenames to be made:");
    rempref.get_tasks().iter().for_each(|task| {
        println!("{task}");
    });
    // TODO case of no tasks should be handled on UI
    println!();
    if flush {
        println!("\nExecuting renames...");
        let (success_count, fail_count) = rempref.execute();
        if fail_count == 0 {
            println!("Renames successful, {success_count} files renamed!\n");
        } else if success_count == 0 {
            println!("All {fail_count} renames failed:");
            rempref.get_failed_tasks().iter().for_each(|failed_task| {
                println!("{failed_task}");
            });
        } else {
            println!(
                "{} renames are successful, but {} renames failed:",
                success_count, fail_count
            );
            rempref.get_failed_tasks().iter().for_each(|failed_task| {
                println!("{failed_task}");
            });
        }
        println!()
    } else {
        println!("Run with -r flag to execute renames\n");
    }
    Ok(())
}
