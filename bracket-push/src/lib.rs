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
        let mut queue = Vec::new();

        let mut pairs = HashMap::new();
        pairs.insert('[', ']');
        pairs.insert('{', '}');
        pairs.insert('(', ')');

        let sanitized_string = self
            .s
            .chars()
            .filter(|c| pairs.iter().any(|(left, right)| *c == *left || *c == *right));

        for c in sanitized_string {
            if pairs.get(&c).is_some() {
                queue.push(c)
            } else {
                if pairs.get(
                    &(queue.pop().unwrap_or_default())
                ).ne(&Some(&c)) {
                    return false;
                }
            }
        }

        queue.is_empty()
    }
}
