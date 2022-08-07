use crate::cli::Operation;
use commons::file::traits::Runnable;

mod cli;
mod logic;

fn main() {
    Operation::run("move");
}
