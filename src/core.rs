pub fn encode<'a>(stream: &'a str, key: usize) -> String {
    let mut after = String::new();

    for byte in stream.as_bytes() {
        if *byte < 97 || *byte > 122 {
            todo!("todo!");
        }
    }

    after
}
