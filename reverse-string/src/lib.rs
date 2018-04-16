pub fn reverse(input: &str) -> String {
    // TODO  use grapheme clusters
    let mut bytesInput = String::from(input).into_bytes();
    bytesInput.reverse();

    return String::from_utf8(Vec::from(bytesInput)).unwrap()
}
