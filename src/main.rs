use crate::cli::{get_arguments, run};

mod cli;

fn main() {
    let _ = get_arguments();

    let response = run("localhost:8080".into());
}
