use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rempref")]
/// A file prefix removal utility
pub struct Args {
    // pub debug: bool,
    /// Pass to make the file renames, otherwise it only does a dry run
    #[structopt(short, long)]
    pub remove_prefix: bool,
    /// The length of the prefix to remove
    #[structopt(short, long)]
    pub prefix_length: u8,
    // pub extensions: Option<Vec<String>>,
    // pub pattern: Option<String>,
    // pub recursive: bool,
    // pub show_similarities: bool
    // pub path: Option<PathBuf>
    // pub restore_point: bool,
    // pub restore_on_failure: bool,
    // pub wait_confirmation: bool,
}
