pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .filter(|c| c.is_alphabetic() || *c == ' ' || *c == '-')
        .collect::<String>()
        .split(" ")
        .flat_map(|word| word.split("-"))
        .map(|word| {
            let mut chars = word.chars();
            let char1 = chars.nth(0).unwrap();
            if word.chars().fold(true, |acc, c| acc && c.is_uppercase()) {
                return char1.to_string();
            }

            let mut other_caps = chars.filter(|c| c.is_uppercase()).collect::<Vec<char>>();

            other_caps.insert(0, char1);

            other_caps.into_iter().collect::<String>()
        })
        .collect::<String>()
        .to_uppercase()
}
