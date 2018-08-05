pub fn find() -> Option<u32> {
    for i in 500..999 {
        for j in 1..(1000 - i) {
            let k = 1000 - i - j;

            if i * i == j * j + k * k {
                return Option::from(i * j * k);
            }
        }
    }

    return Option::None
}
