use rand::prelude::*;
use std::{
    env, fs,
    path::{PathBuf, MAIN_SEPARATOR},
};

const CHOSEN_DIR: &str = "_chosen";

fn main() {
    let cwd = env::current_dir().expect("could not get working directory");
    let entries = fs::read_dir(cwd.clone())
        .expect("could not get entries in directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.metadata().expect("cannot read metadata").is_file())
        .collect::<Vec<_>>();
    let rand = rand::thread_rng().gen_range(0..entries.len());
    let rand = entries.get(rand).expect("invalid index");
    let _ = fs::create_dir(CHOSEN_DIR);
    let to = PathBuf::from(format!(
        "{}{MAIN_SEPARATOR}{CHOSEN_DIR}{MAIN_SEPARATOR}{}",
        cwd.display(),
        rand.file_name().to_string_lossy()
    ));
    println!(
        "Chosen file is: {}",
        to.file_name()
            .expect("could not extract filename")
            .to_string_lossy()
    );
    fs::rename(rand.path(), to).expect("could not move chosen file");
}
