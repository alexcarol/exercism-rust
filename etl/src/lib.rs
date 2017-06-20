use std::collections::BTreeMap;

pub fn transform(old: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut map: BTreeMap<char, i32> = BTreeMap::new();

    for (points, letters) in old {
        for letter in letters {
            map.insert(letter.to_lowercase().next().unwrap(), *points);
        }
    }

    map
}
