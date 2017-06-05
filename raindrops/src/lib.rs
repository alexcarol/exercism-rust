pub fn raindrops(i: i64) -> String {
    let mut ret = String::from("");

    if i % 3 == 0 {
        ret.push_str("Pling")
    }

    if i % 5 == 0 {
        ret.push_str("Plang")
    }

    if i % 7 == 0 {
        ret.push_str("Plong")
    }

    if ret.len() > 0 {
        return ret
    }

    format!("{}", i)
}
