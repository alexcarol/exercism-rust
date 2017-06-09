pub fn score(s: &str) -> i64 {
    s.to_uppercase().chars().fold(0, |sum, c| sum + letterscore(c))
}

pub fn letterscore(c: char) -> i64 {
    match c {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' =>  8,
        'Q' | 'Z' =>  10,
        _ => 0
    }
}
