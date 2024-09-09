use crate::EXIT_FAILURE;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::process;

pub fn get_file<'a>(f: &'a str) -> File {
    let file: Result<_, _>;

    match &f[f.len() - 5..] {
        ".ciph" => file = File::open(f),
        _ => {
            eprintln!(
                "{}",
                Error::new(ErrorKind::Other, "invalid filetypes, expected *.ciph")
            );
            process::exit(EXIT_FAILURE);
        }
    }

    match file {
        Ok(found) => found,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(EXIT_FAILURE);
        }
    }
}
