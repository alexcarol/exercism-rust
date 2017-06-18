use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut h = HashMap::new();
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

        if h.contains_key(AsRef::<str>::as_ref(&stringized)) {
            let x = h.get_mut(AsRef::<str>::as_ref(&stringized)).unwrap();
            *x += 1;
        } else {
            h.insert(stringized, 1);
        }
    }

    h
}
