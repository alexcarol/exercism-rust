pub fn verse(i: i64) -> String {
    if i == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    }

    if i == 1 {
        return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
    }

    if i == 2 {
        return String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n")
    }

    return format!("{amount} bottles of beer on the wall, {amount} bottles of beer.\nTake one down and pass it around, {amount2} bottles of beer on the wall.\n", amount=i, amount2=(i - 1))
}

pub fn sing(mut from: i64, to: i64) -> String {
    let mut value = String::new();
    while from >= to {
        value += verse(from).as_str();
        if from != to {
            value += "\n";
        }

        from -= 1
    }

    value
}