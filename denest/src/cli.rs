use crate::logic::{Config, Denest};
use commons::file::traits::{InputArgs, Runnable};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "denest")]
/// Nested file hoister utility
/// It moves nested files from folders in the working directory, to the working directory
pub struct Args {
    /// Pass to move the files, otherwise it only does a dry run
    #[structopt(short, long)]
    pub do_moves: bool,
    /// Specify file extensions to move to the working directory
    #[structopt(short, long)]
    pub extensions: Vec<String>,
    /// Depth of the recursive search
    #[structopt(long)]
    pub depth: Option<u8>,
    /// Cleanup the empty folders after
    #[structopt(short, long)]
    pub cleanup: bool,
    /// Specify the working directory
    #[structopt(long)]
    pub working_dir: Option<PathBuf>,
    // pub savepoint: bool, // save a csv with information to restore the moved files
    // pub load_savepoint: bool // restore the moved files on failure
}

impl InputArgs for Args {
    fn working_dir(&self) -> Option<PathBuf> {
        self.working_dir.clone()
    }

    fn do_exec(&self) -> bool {
        self.do_moves
    }
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config::new(args.extensions, args.depth, args.cleanup)
    }
}

pub struct Operation;

impl Runnable<Args, Config, Denest> for Operation {
    fn name() -> String {
        "=== Denest ===".to_string()
    }

    fn verb() -> String {
        "move".to_string()
    }
}
