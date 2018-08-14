#[derive(PartialEq,  Debug)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T>(v: &[T], w: &[T]) -> Comparison
where T: PartialEq {
    if v.len() == w.len() {
        if w
            .iter()
            .enumerate()
            .all(|(pos, value)| *value == v[pos]) {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }

    } else if v.len() < w.len() {
        if w
            .iter()
            .enumerate()
            .any(|(pos, _)| pos + v.len() <= w.len() && sublist(v, &w[pos .. pos + v.len()]) == Comparison::Equal) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    } else {
        if sublist(w, v) == Comparison::Sublist {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
    }
}