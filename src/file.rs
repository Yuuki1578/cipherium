use std::fs::File;
use std::io::{Error, ErrorKind};
use std::process;

pub fn get_file<'a>(f: &'a str) -> File {
    let mut file: Result<File, Error>;

    match &f[f.len() - 5..] {
        ".ciph" => file = File::open(f),
        _ => {
            eprintln!(
                "{}",
                Error::new(ErrorKind::Other, "invalid filetypes, expected *.ciph")
            );
            process::exit(1);
        }
    }

    match file {
        Ok(found) => found,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(2);
        }
    }
}
