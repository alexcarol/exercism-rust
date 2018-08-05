pub fn find() -> Option<u32> {
    for i in 1..1000 {
        for j in 1..1000 {
            if i + j < 1000 {
                let k = 1000 - i - j;
                let mut numbers = vec![i, j, k];
                numbers.sort();

                if numbers[0] * numbers[0] + numbers[1] * numbers[1] == numbers[2] * numbers[2] {
                    println!("{}, {}, {}", i, j, k);
                    return Option::from(i * j * k);
                }
            }
        }
    }

    return Option::None
}
