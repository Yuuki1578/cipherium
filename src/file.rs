use crate::EXIT_FAILURE;
use std::fs::File;
use std::process;

pub fn get_file<'a>(f: &'a str) -> File {
    let file = File::open(f);

    match file {
        Ok(found) => found,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(EXIT_FAILURE);
        }
    }
}
