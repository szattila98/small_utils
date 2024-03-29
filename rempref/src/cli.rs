use crate::logic::{Config, Rempref};
use commons::file::traits::{InputArgs, Runnable};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rempref")]
/// A file prefix removal utility
pub struct Args {
    /// The length of the prefix to remove
    #[structopt(short, long)]
    pub prefix_length: u8,
    /// Pass to make the file renames, otherwise it only does a dry run
    #[structopt(short, long)]
    pub do_renames: bool,
    /// Specify file extensions to remove the prefix from
    #[structopt(short, long)]
    pub extensions: Vec<String>,
    /// Recursively search all files in the working directory
    #[structopt(short, long)]
    pub recursive: bool,
    /// Specify the working directory
    #[structopt(long)]
    pub working_dir: Option<PathBuf>,
    // pub savepoint: bool, // save a csv with information to restore the renames
    // pub load_savepoint: bool // restore the renames on failure
    // pub file_pattern: Option<Patterns>, // basic patterns enum with custom option
    // pub prefix_pattern: Option<String>, // eg remove prefixes that match this pattern
    // pub similarities: bool // search for similarities between files and create tasks based on those
}

impl InputArgs for Args {
    fn working_dir(&self) -> Option<PathBuf> {
        self.working_dir.clone()
    }

    fn do_exec(&self) -> bool {
        self.do_renames
    }
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config::new(args.prefix_length, args.extensions, args.recursive)
    }
}

pub struct Operation;

impl Runnable<Args, Config, Rempref> for Operation {
    fn name() -> String {
        "=== Rempref ===".to_string()
    }

    fn verb() -> String {
        "rename".to_string()
    }
}
