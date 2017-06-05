pub fn reply(s: &str) -> &'static str {
    if s == "" {
        return "Fine. Be that way!"
    }

    let (_, last_char) = s.split_at(s.len() - 1);
    if last_char == "?" {
        return "Sure."
    }

    if s == s.to_uppercase() {
        return "Whoa, chill out!"
    }

    "Whatever."
}