use pathdiff::diff_paths;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn walkdir(root: &Path, depth: Option<usize>) -> Vec<PathBuf> {
    let dir_iter = if let Some(depth) = depth {
        WalkDir::new(root).max_depth(depth)
    } else {
        WalkDir::new(root)
    };
    dir_iter
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().into_path())
        .collect::<Vec<_>>()
}

pub fn read_files(root: &Path, depth: Option<usize>) -> Vec<PathBuf> {
    walkdir(root, depth)
        .into_iter()
        .filter(|path| path.is_file())
        .collect::<Vec<_>>()
}

pub fn read_dirs(root: &Path, depth: Option<usize>) -> Vec<PathBuf> {
    walkdir(root, depth)
        .into_iter()
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>()
}

pub fn filter_by_extension(files: Vec<PathBuf>, extensions: &Vec<String>) -> Vec<PathBuf> {
    if extensions.is_empty() {
        return files;
    }
    files
        .into_iter()
        .filter_map(|file| {
            extensions
                .contains(&file.extension()?.to_string_lossy().to_string())
                .then(|| file)
        })
        .collect::<Vec<_>>()
}

pub fn is_in_working_dir(working_dir: &PathBuf, file: &PathBuf) -> bool {
    match diff_paths(file, working_dir) {
        Some(diff) => diff.iter().count() != 1,
        None => false,
    }
}
