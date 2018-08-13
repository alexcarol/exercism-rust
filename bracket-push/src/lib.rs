use std::collections::HashMap;

pub struct Brackets {
    s: String,
}

impl<'a> From<&'a str> for Brackets {
    fn from(input: &str) -> Self {
        Brackets { s: input.to_string() }
    }
}

impl Brackets {
    pub fn are_balanced(&self) -> bool {
        is_string_balanced(self.s.clone())
    }
}

fn is_string_balanced(s: String) -> bool {
    let mut queue = Vec::new();
    let allowed_values = "[]{}()";
    let mut pairs = HashMap::new();
    pairs.insert('[', ']');
    pairs.insert('{', '}');
    pairs.insert('(', ')');

    let sanitized_string  = s
        .chars()
        .filter(|c| allowed_values.chars().any(|allowed_value| *c == allowed_value));

    for c in sanitized_string {
        if pairs.get(&c).is_some() {
            queue.push(c)
        } else {
            if pairs.get(
            &(queue.pop().unwrap_or_default())
            ).ne(&Some(&c)) {
                return false
            }
        }
    }

    return queue.is_empty()
}