pub fn lsp(numbers: &str, size: usize) -> Result<u32, bool> {
    if size > numbers.len() {
        return Result::Err(false);
    }
    let num_vec: Vec<char> = numbers.chars().collect();
    let mut max = 0;
    for i in 0..(numbers.len() - size + 1) {
        let mut product = 1;
        for j in 0..size {
            let digit = num_vec[i + j].to_digit(10);
            if digit == None {
                return Result::Err(false);
            }
            product *= digit.unwrap();
        }
        if product > max {
            max = product;
        }
    }

    Result::Ok(max)
}
