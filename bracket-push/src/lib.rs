use std::collections::HashMap;

pub struct Brackets<'a> {
    s: &'a str,
}

impl<'a> From<&'a str> for Brackets<'a> {
    fn from(input: &'a str) -> Self {
        Brackets { s: input }
    }
}

impl<'a> Brackets<'a> {
    pub fn are_balanced(&self) -> bool {
        let mut pairs = HashMap::new();
        pairs.insert('[', ']');
        pairs.insert('{', '}');
        pairs.insert('(', ')');

        let mut queue = Vec::new();

        let sanitized_string = self
            .s
            .chars()
            .filter(|c| pairs.iter().any(|(left, right)| *c == *left || *c == *right));

        for c in sanitized_string {
            let option = pairs.get(&c);

            if option.is_some() {
                queue.push(option.unwrap());
            } else if !queue.pop().map(|queued| c == *queued).unwrap_or(false) {
                return false
            }
        }

        queue.is_empty()
    }
}
