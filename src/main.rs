mod exit_code;
mod range;
mod scanner;

use std::{ env, path::Path, process };
use range::{ Range, RangeError };
use scanner::Scanner;

const ACCEPTED_ARG_COUNT: usize = 1;

fn main() {
    let args = env::args().collect();
    let range = Range::new(&args, 2, 2);
    match range.to_iter() {
        Ok(args) => {
            let root_dir = Path::new(&args[1]);
            if root_dir.is_dir() {
                Scanner::new(root_dir).scan();
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
