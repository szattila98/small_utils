use cli::Args;
use error::RemPrefError;
use logic::Rempref;
use structopt::StructOpt;

mod cli;
mod error;
mod logic;

fn main() -> Result<(), RemPrefError> {
    let args = Args::from_args();
    let flush = args.execute_renames;
    let mut rempref = Rempref::init(args.into())?;
    println!("Renames to be made:");
    rempref.get_tasks().iter().for_each(|task| {
        println!("{} -> {}", task.from, task.to);
    });
    if flush {
        println!("Executing renames...");
        let (success_count, fail_count) = rempref.execute();
        if fail_count == 0 {
            println!("Renames successful, {success_count} files renamed!");
        } else if success_count == 0 {
            println!("All {fail_count} renames failed:");
            rempref.get_failed_tasks().iter().for_each(|task| {
                println!("{} -> {}", task.from, task.to);
            });
        } else {
            println!(
                "{} renames are successful, but {} renames failed:",
                success_count, fail_count
            );
            rempref.get_failed_tasks().iter().for_each(|task| {
                println!("{} -> {}", task.from, task.to);
            });
        }
    }
    Ok(())
}
