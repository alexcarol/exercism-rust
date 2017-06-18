use std::collections::HashMap;

pub fn decode(s: &str) -> &str {
    s
}

pub fn encode(s: &str) -> String {
    let plain: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let cipher: Vec<char> = "zyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcba"
        .chars()
        .collect();

    let mut map: HashMap<char, char> = HashMap::new();
    for i in 0..(plain.len()) {
        map.insert(plain[i], cipher[i]);
    }

    s.chars()
        .filter(|c| *c != ' ' && *c != ',' && *c != '.')
        .fold(String::from(""), |acc, c| {
            let extra = if (acc.len() + 1) % 6 == 0 { " " } else { "" };
            let map_found = map.get(&c);
            let res_char = if map_found == None {
                &c
            } else {
                map_found.unwrap()
            };
            format!("{}{}{}", acc, extra, res_char)
        })
}
