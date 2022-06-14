use crate::logic::{Config, FailedRemprefTask, RemPrefTask};
use std::fmt::Display;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rempref")]
/// A file prefix removal utility
pub struct Args {
    // pub debug: bool,
    /// The length of the prefix to remove
    #[structopt(short, long)]
    pub prefix_length: u8,
    /// Pass to make the file renames, otherwise it only does a dry run
    #[structopt(short, long)]
    pub do_renames: bool,
    /// Specify file extensions to remove the prefix from
    #[structopt(short, long)]
    pub extensions: Vec<String>,
    // pub file_pattern: Option<String>,
    // pub removal_pattern: Option<String>,
    // pub recursive: bool,
    // pub show_similarities: bool
    // pub path: Option<PathBuf>
    // pub restore_point: bool,
    // pub restore_on_failure: bool,
    // pub wait_confirmation: bool,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config {
            prefix_length: args.prefix_length,
            extensions: args.extensions,
        }
    }
}

impl Display for RemPrefTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from.display(), self.to.display()) // TODO working dir should not be shown
    }
}

impl Display for FailedRemprefTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.file_path.display(), self.reason) // TODO working dir should not be shown
    }
}
