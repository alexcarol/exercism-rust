pub struct Roman {

}

impl Roman {
    // This only works for number up to 9999, it's enough though (up to 3000 as the readme says)
    pub fn from(n: u64) -> String {
        format!(
            "{}{}{}{}",
            Roman::generic((n/1000) % 10, "M", "MMMMM", "MMMMMMMMMM"),
            Roman::generic((n/100) % 10, "C", "D", "M"),
            Roman::generic((n / 10) % 10, "X", "L", "C"),
            Roman::generic(n % 10, "I", "V", "X")
        )
    }

    fn generic(n: u64, one: &str, five: &str, ten: &str) -> String {
        if n <= 3 {
            return String::from(one).repeat(n as usize)
        }

        if n >= 5 && n <= 8 {
            return format!("{}{}", five, String::from(one).repeat((n - 5) as usize))
        }
        String::from(match n {
            4 => format!("{}{}", one, five),
            9 => format!("{}{}", one, ten),
            _ => String::from(""),
        })
    }
}