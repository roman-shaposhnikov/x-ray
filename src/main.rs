mod shared;
mod project;
mod response;

use std::{ env, path::Path, process };
use shared::range::{ Range, RangeError };
use shared::exit_code;
use project::Project;

const ACCEPTED_ARG_COUNT: usize = 1;

fn main() {
    let args = env::args().collect();
    let range = Range::new(&args, 2, 2);
    match range.to_iter() {
        Ok(args) => {
            let root_entry = Path::new(&args[1]);
            if root_entry.is_file() {
                if let Ok(responce) = Project::new(root_entry).fan() {
                    println!("{responce:?}");
                };
            }
        }
        Err(error) => {
            report_args_error(error, ACCEPTED_ARG_COUNT);
            process::exit(exit_code::CLI_ARGS_ERROR);
        }
    }
}

fn report_args_error(error: RangeError, expected: usize) {
    match error {
        RangeError::Over(count) => {
            eprintln!("Too many arguments! Expected: {}, Passed: {}", expected, count);
        }
        RangeError::Under(_) => {
            eprintln!("Missing path to root directory!");
        }
    }
}
