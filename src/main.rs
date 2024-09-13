pub mod cli;
pub mod file;
pub mod utils;

pub const ARGV_0: usize = 0;
pub const ARGV_1: usize = 1;
pub const ARGV_2: usize = 2;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

use std::io;
pub use std::process;

fn main() -> io::Result<()> {
    let msg = String::from("awang");
    let template: String;

    println!("{}", {
        template = utils::encode(&msg[..], 18);
        template.clone()
    });
    println!("{}", utils::decode(&template[..], 18));

    Ok(())
}
