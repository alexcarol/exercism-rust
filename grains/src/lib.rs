pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    if s == 1 {
        return 1
    }

    square(s - 1) * 2
}

pub fn total() -> u64 {
    let mut sum = 0;
    for i in 1..65 {
        sum += square(i);
    }

    sum
}
