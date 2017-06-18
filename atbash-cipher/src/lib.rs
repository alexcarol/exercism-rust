use std::collections::HashMap;


pub fn decode(s: &str) -> String {
    let cipher: Vec<char> = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"
        .chars()
        .collect();
    let plain: Vec<char> = "zyxwvutsrqponmlkjihgfedcbaZYXWVUTSRQPONMLKJIHGFEDCBA"
        .chars()
        .collect();

    generic(s, plain, cipher)
}

pub fn encode(s: &str) -> String {
    let plain: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let cipher: Vec<char> = "zyxwvutsrqponmlkjihgfedcbazyxwvutsrqponmlkjihgfedcba"
        .chars()
        .collect();

    generic(s, plain, cipher).chars().fold(
        String::from(""),
        |acc, c| if (acc.len() + 1) % 6 ==
            0
        {
            format!("{}{}{}", acc, " ", c)
        } else {
            format!("{}{}", acc, c)
        },
    )
}

fn generic(s: &str, plain: Vec<char>, cipher: Vec<char>) -> String {
    let mut map: HashMap<char, char> = HashMap::new();
    for i in 0..(plain.len()) {
        map.insert(plain[i], cipher[i]);
    }

    s.chars()
        .filter(|c| *c != ' ' && *c != ',' && *c != '.')
        .fold(String::from(""), |acc, c| {
            let map_found = map.get(&c);
            let res_char = if map_found == None {
                if !c.is_numeric() {
                    return acc;
                }

                &c
            } else {
                map_found.unwrap()
            };
            format!("{}{}", acc, res_char)
        })
}
