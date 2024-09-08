pub mod cli;
pub mod core;
pub mod file;

const ARGV0: usize = 0;
const ARGV1: usize = 1;
const ARGV2: usize = 2;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

use cli::Args;
use std::io;
use std::process;

fn main() -> io::Result<()> {
    let fixed_args = Args::parse(3);

    fixed_args.compare(ARGV0, "help", || {
        println!("You asking for Help!");
        process::exit(EXIT_SUCCESS);
    });

    fixed_args.compare(ARGV0, "version", || {
        println!("You asking for Version!");
        process::exit(EXIT_FAILURE);
    });

    fixed_args.compare(ARGV0, "encode", || {
        if let Some(_path) = fixed_args.get(ARGV1) {
            if let Some(_key) = fixed_args.get(ARGV2) {
                println!("You have file and key!");
            } else {
                println!("You only have file!");
            }
        } else {
            println!("You have nothing!");
        }
    });

    Ok(())
}
