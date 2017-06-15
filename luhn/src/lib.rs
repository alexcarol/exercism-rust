pub fn is_valid(s: &str) -> bool {
    let mut i = 1;
    let mut sum = 0;
    for c in s.chars().rev() {
        if !c.is_digit(10) {
            if c == ' ' {
                continue;
            }

            return false;
        }
        let digit = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            let double = digit * 2;
            if double > 9 {
                sum += double - 9
            } else {
                sum += double
            }
        } else {
            sum += digit;
        }

        i += 1
    }
    if i <= 2 {
        return false;
    }

    print!("{}", sum);

    sum % 10 == 0
}
