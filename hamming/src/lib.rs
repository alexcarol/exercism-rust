pub fn hamming_distance(a: &str, b: &str) -> Result<i64, i64> {
    if a.len() != b.len() {
        return Err(0)
    }
    if a == b {
        return Ok(0)
    }

    let chars_a = a.chars();
    let mut chars_b = b.chars();

    let mut distance = 0;
    for anth in chars_a {
        let bnth = chars_b.next().unwrap();
        if anth != bnth {
            distance += 1;
        }
    }

    Ok(distance)
}