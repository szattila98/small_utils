use glob::glob;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rempref", about = "A file prefix removal utility.")]
struct Args {
    // debug: bool,
    #[structopt(short, long)]
    modify: bool,
    #[structopt(short, long)]
    chars: u8,
    // extensions: Option<Vec<String>>,
    // pattern: Option<String>,
    // recursive: bool,
    // show_similarities: bool
    // path: Option<PathBuf>
}

fn main() {
    let args = Args::from_args();

    let working_dir =
        std::env::current_dir().expect("Permission denied or current directory does no exist");
    let working_dir = working_dir
        .into_os_string()
        .into_string()
        .expect("Current directory string invalid");

    let glob_pattern = format!("{}/*", working_dir);
    let mut files = vec![];
    for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => files.push(path),
            Err(e) => println!("{e}"),
        }
    }
    let files = files
        .iter()
        .filter(|file| file.is_file())
        .collect::<Vec<_>>();
    let prefix_count = args.chars.into();
    let files_str = files
        .iter()
        .map(|file| format!("{}", file.display()))
        .map(|file| format!("{} -> {}", file, &file[prefix_count..]))
        .collect::<Vec<_>>();

    println!("{}", files_str.join("\n"));

    if args.modify {}
}
