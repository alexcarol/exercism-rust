pub fn encode(s: &str) -> String {
    let mut res = String::new();

    let mut current_streak_char = '-';
    let mut amount = 0;
    for current_char in s.chars() {
        if current_streak_char != current_char {
            if amount > 0 {
                if amount == 1 {
                    res.push(current_streak_char);
                } else {
                    res.push_str(format!("{}{}", amount, current_streak_char).as_str());
                }
                amount = 0;
            }
        }

        current_streak_char = current_char;
        amount += 1;
    }

    if amount > 0 {
        if amount == 1 {
            res.push(current_streak_char);
        } else {
            res.push_str(format!("{}{}", amount, current_streak_char).as_str());
        }
    }


    res
}

pub fn decode(s: &str) -> String {
    let mut res = String::new();
    let mut multiplier = 0;

    for singlechar in s.chars() {
        if singlechar.is_digit(10) {
            multiplier = multiplier * 10 + singlechar.to_digit(10).unwrap();
        } else {
            if multiplier == 0 {
                res.push(singlechar);
            } else {
                for _ in 0..multiplier {
                    res.push(singlechar);
                }
            }
            multiplier = 0;
        }
    }

    res
}