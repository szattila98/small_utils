use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rempref", about = "A file prefix removal utility.")]
pub struct Args {
    // pub debug: bool,
    #[structopt(short, long)]
    pub modify: bool,
    #[structopt(short, long)]
    pub prefix_len: u8,
    // pub extensions: Option<Vec<String>>,
    // pub pattern: Option<String>,
    // pub recursive: bool,
    // pub show_similarities: bool
    // pub path: Option<PathBuf>
}
