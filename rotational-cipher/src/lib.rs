pub fn rotate(s: &str, i: u8) -> String {
    s.chars()
        .map(|c| {
            if !c.is_alphabetic() {
                return c;
            }
            if c.is_lowercase() {
                char::from('a' as u8 + (c as u8 + i - 'a' as u8) % 26)
            } else {
                char::from('A' as u8 + (c as u8 + i - 'A' as u8) % 26)
            }
        })
        .collect()
}
