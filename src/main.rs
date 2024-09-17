pub mod cli;
pub mod file;
pub mod utils;

/// # Major version of the program
/// used in src/main.rs 34:37
pub const VERSION: &'static str = "0.1.0";

/// # C argv[0] representation
pub const ARGV_0: usize = 0;

/// # C argv[1] representation
pub const ARGV_1: usize = 1;

/// # C argv[2] representation
pub const ARGV_2: usize = 2;

/// # C stdlib/EXIT_SUCCCESS macro representation
pub const EXIT_SUCCESS: i32 = 0;

/// # C stdlib/EXIT_FAILURE macro representation
pub const EXIT_FAILURE: i32 = 1;

use cli::Args;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    let args = Args::parse(3);

    if args.len() == 0 {
        println!("{}", cli::help());
        process::exit(EXIT_FAILURE);
    }

    args.compare(0, "--help", "-h", || {
        println!("{}", cli::help());
        process::exit(EXIT_SUCCESS);
    });

    args.compare(0, "--version", "-v", || {
        println!("{}", cli::version(VERSION));
        process::exit(EXIT_SUCCESS);
    });

    args.compare(0, "--encode", "-e", || {
        let (arg1, arg2) = (args.get(1), args.get(2));
        let (mut file, key): (File, u8);
        let mut buffer = String::new();

        match (arg1, arg2) {
            (Some(found_f), Some(found_k)) => {
                file = file::get_file(found_f);
                key = match found_k.parse::<u8>() {
                    Ok(val) => val,
                    Err(error) => {
                        eprintln!("{}", error);
                        process::exit(EXIT_FAILURE);
                    }
                };
            }

            (Some(_), None) => {
                eprintln!("missing key for shift right");
                process::exit(EXIT_FAILURE);
            }

            _ => {
                eprintln!("missing file to be encrypted");
                process::exit(EXIT_FAILURE);
            }
        }

        let _ = match file.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("{}", error);
                process::exit(EXIT_FAILURE);
            }
        };

        println!("{}", utils::encode(buffer.trim(), key));
        process::exit(EXIT_SUCCESS);
    });

    args.compare(0, "--decode", "-d", || {
        let (arg1, arg2) = (args.get(1), args.get(2));
        let (mut file, key): (File, u8);
        let mut buffer = String::new();

        match (arg1, arg2) {
            (Some(found_f), Some(found_k)) => {
                file = file::get_file(found_f);
                key = match found_k.parse::<u8>() {
                    Ok(val) => val,
                    Err(error) => {
                        eprintln!("{}", error);
                        process::exit(EXIT_FAILURE);
                    }
                };
            }

            (Some(_), None) => {
                eprintln!("missing key for shift left");
                process::exit(EXIT_FAILURE);
            }

            _ => {
                eprintln!("missing file to be decrypted");
                process::exit(EXIT_FAILURE);
            }
        }

        let _ = match file.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("{}", error);
                process::exit(EXIT_FAILURE);
            }
        };

        println!("{}", utils::decode(buffer.trim(), key));
        process::exit(EXIT_SUCCESS);
    });
}
