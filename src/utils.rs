pub fn encode(stream: &str, key: u8) -> String {
    stream
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() { b'a' } else { b'A' };

                ((c as u8 - base + key) % 26 + base) as char
            } else if c.is_numeric() {
                let base = b'0';

                ((c as u8 - base + key) % 10 + base) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn decode(stream: &str, key: u8) -> String {
    stream
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() {
                    b'a' as i32
                } else {
                    b'A' as i32
                };
                let result = (c as u8 as i32 - base - key as i32 + 26) % 26 + base;
                char::from(result as u8)
            } else if c.is_numeric() {
                let base = b'0' as i32;
                let result = (c as u8 as i32 - base - key as i32 + 10) % 10 + base;
                char::from(result as u8)
            } else {
                c
            }
        })
        .collect()
}
