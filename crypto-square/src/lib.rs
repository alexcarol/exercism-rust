use std::string::String;

pub fn encrypt(input: &str) -> String {
    let normalized: Vec<char> = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && !c.is_ascii_punctuation())
        .collect()
    ;

    let r = sqrt(normalized.len());
    let c = if r * r == normalized.len() {
        r
    } else {
        r + 1
    };

    let mut encrypted = String::new();

    for i in 0..c {
        if i != 0 {
            encrypted.push(' ');
        }
        for j in 0..r {
            let position = i + j * c;
            encrypted.push(*normalized.get(position).unwrap_or(&' '));
        }
    }

    encrypted
}

fn sqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
}