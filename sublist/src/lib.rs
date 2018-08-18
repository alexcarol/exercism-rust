#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T>(v: &[T], w: &[T]) -> Comparison
    where T: PartialEq {
    if v.len() == w.len() {
        if v == w {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else if v.len() < w.len() {
        if is_sublist(v, w) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    } else {
        if is_sublist(w, v) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
    }
}

fn is_sublist<T>(v: &[T], w: &[T]) -> bool
    where T: PartialEq {
    v.is_empty() || w.windows(v.len()).any(|w_window| v == w_window)
}
