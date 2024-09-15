pub fn encode<'l>(stream: &'l str, key: u8) -> String {
    stream
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() {
                    'a' as u8
                } else {
                    'A' as u8
                };

                ((c as u8 - base + key) % 26 + base) as char
            } else if c.is_numeric() {
                let base = '0' as u8;

                ((c as u8 - base + key) % 10 + base) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn decode<'l>(stream: &'l str, key: u32) -> String {
    stream
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() {
                    'a' as u8
                } else {
                    'A' as u8
                };

                let uint = c as u8 as u32;

                char::from_u32((uint - base as u32 - key + 26) % 26 + base as u32).unwrap_or(c)
            } else if c.is_numeric() {
                let uint = c as u8 as u32;
                let base = '0' as u8 as u32;

                char::from_u32((uint - base as u32 - key + 10) % 10 + base as u32).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}
