pub fn is_pangram(s: &str) -> bool {
    let letters = "abcdefghijklmnopqrstuvwxyz";

    let lowercase_s = s.to_lowercase();
    for c in letters.chars() {
        if !lowercase_s.contains(c) {
            return false;
        }
    }

    true
}
