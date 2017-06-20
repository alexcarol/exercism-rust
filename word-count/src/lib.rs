use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut h: HashMap<String, u32> = HashMap::new();
    let split = s.split(" ");
    for s in split {
        let stringized = String::from(s)
            .replace(",", "")
            .replace(":", "")
            .replace("!", "")
            .replace("&", "")
            .replace("@", "")
            .replace("~", "")
            .replace("$", "")
            .replace("%", "")
            .replace("^", "")
            .to_lowercase();
        if stringized == "" {
            continue;
        }

        *h.entry(stringized).or_insert(0) += 1;
    }

    h
}
