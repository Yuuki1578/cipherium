use crate::{process, EXIT_FAILURE};
use std::num::Wrapping;

pub type Wrap = Wrapping<u8>;

pub fn wrap(data: u8) -> Wrap {
    Wrapping(data)
}

pub trait Ranged {
    fn in_range(&self, start: u8, end: u8) -> bool;
}

impl Ranged for u8 {
    fn in_range(&self, start: u8, end: u8) -> bool {
        if self >= &start || self <= &end {
            return true;
        }

        false
    }
}

#[derive(Debug)]
pub enum CharsKind {
    LowerCase,
    UpperCase,
    Numeric,
    Undefined,
}

impl CharsKind {
    pub fn formula_encode(&self, byte: u8, key: u8) -> u8 {
        match self {
            CharsKind::LowerCase => {
                let wrap_byte = wrap(byte);
                let wrap_init = wrap(b'a');
                let wrap_key = wrap(key);
                let wrap_modulo = wrap(26);

                { (Wrapping((wrap_byte - wrap_init).0) + wrap_key) % wrap_modulo + wrap_init }.0
            }

            CharsKind::UpperCase => {
                let wrap_byte = wrap(byte);
                let wrap_init = wrap(b'A');
                let wrap_key = wrap(key);
                let wrap_modulo = wrap(26);

                { (Wrapping((wrap_byte - wrap_init).0) + wrap_key) % wrap_modulo + wrap_init }.0
            }

            CharsKind::Numeric => {
                let wrap_byte = wrap(byte);
                let wrap_init = wrap(b'0');
                let wrap_key = wrap(key);
                let wrap_modulo = wrap(10);

                { (Wrapping((wrap_byte - wrap_init).0) + wrap_key) % wrap_modulo + wrap_init }.0
            }

            CharsKind::Undefined => byte,
        }
    }

    pub const fn formula_decode(&self, byte: u8, key: u8) -> u8 {
        match self {
            CharsKind::LowerCase => (byte - b'a' - key + 26) % 26 + b'a',
            CharsKind::UpperCase => (byte - b'A' - key + 26) % 26 + b'A',
            CharsKind::Numeric => (byte - b'0' - key + 10) % 10 + b'0',
            CharsKind::Undefined => byte,
        }
    }
}

pub fn encode<'a>(stream: &'a str, key: u8) -> String {
    let mut as_vec = stream.to_string().into_bytes();

    for byte in as_vec.iter_mut() {
        if byte.in_range(97, 122) {
            *byte = CharsKind::LowerCase.formula_encode(*byte, key);
            continue;
        }

        if byte.in_range(65, 90) {
            *byte = CharsKind::UpperCase.formula_encode(*byte, key);
            continue;
        }

        if byte.in_range(48, 57) {
            *byte = CharsKind::Numeric.formula_encode(*byte, key);
            continue;
        }

        *byte = CharsKind::Undefined.formula_encode(*byte, key);
    }

    match String::from_utf8(as_vec) {
        Ok(success) => success,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(EXIT_FAILURE);
        }
    }
}
pub fn decode<'a>(stream: &'a str, key: u8) -> String {
    let mut as_vec = stream.to_string().into_bytes();

    for byte in as_vec.iter_mut() {
        if byte.in_range(97, 122) {
            *byte = CharsKind::LowerCase.formula_decode(*byte, key);
            continue;
        }

        if byte.in_range(65, 90) {
            *byte = CharsKind::UpperCase.formula_decode(*byte, key);
            continue;
        }

        if byte.in_range(48, 57) {
            *byte = CharsKind::Numeric.formula_decode(*byte, key);
            continue;
        }

        *byte = CharsKind::Undefined.formula_decode(*byte, key);
    }

    match String::from_utf8(as_vec) {
        Ok(success) => success,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(EXIT_FAILURE);
        }
    }
}

pub mod tests {
    #[test]
    pub fn test_encode_low_extra() {
        println!(
            "{}",
            super::encode("awang, \nabcdefghijklmnopqrstuvwxyz\t", 17)
        );
    }

    #[test]
    pub fn test_encode_up_and_low() {
        println!(
            "{}",
            super::encode(
                "Awang Destu Pradhana CoYY Zstdlib Zstd Zsh Xsever Xeon XSDL Android ELF HeadeR",
                13
            )
        );
    }

    #[test]
    pub fn test_encode_up() {
        println!(
            "{}",
            super::encode(
                "PROGRAM MAIN\n\tPRINT *, \"HELLO WORLD!\"\nEND PROGRAM MAIN",
                30
            )
        );
    }

    #[test]
    pub fn test_encode_symbols() {
        println!("{}", super::encode("\t\n\r={}{%]^¥®A\n\n\t\r}", 14));
    }
}
