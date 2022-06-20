use crate::logic::Config;
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
    // pub working_dir: Option<PathBuf> // custom working directory
    // pub savepoint: bool, // save a csv with information to restore the moved files
    // pub load_savepoint: bool // restore the moved files on failure
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config::new(args.extensions, args.depth)
    }
}
