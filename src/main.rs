pub mod cli;
pub mod file;
pub mod utils;

pub const ARGV_0: usize = 0;
pub const ARGV_1: usize = 1;
pub const ARGV_2: usize = 2;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

use std::io;
use std::num::Wrapping;
pub use std::process;

fn main() -> io::Result<()> {
    let x = Wrapping(0 as u8);
    let y = Wrapping(1 as u8);

    println!("{}", (x - y).0);

    Ok(())
}
