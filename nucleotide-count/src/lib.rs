use std::collections::HashMap;

pub fn nucleotide_counts(dna_sequence: &str) -> Result<HashMap<char, usize>, &str> {
    let mut map = HashMap::new();
    let valid_nucleotides = "CGTA";
    for element in valid_nucleotides.chars() {
        let result = count(element, dna_sequence);
        if result.is_err() {
            return Result::Err("error");
        }
        map.insert(element, result.unwrap());
    }

    Result::Ok(map)
}

pub fn count(nucleotide: char, dna_sequence: &str) -> Result<usize, &str> {
    let valid_nucleotides = "CGTA";
    if !valid_nucleotides.contains(nucleotide) {
        return Result::Err("invalid nucleotide");
    }

    for element in dna_sequence.chars() {
        if !valid_nucleotides.contains(element) {
            return Result::Err("invalid nucleotide");
        }
    }


    let amount = dna_sequence.chars().filter(|c| *c == nucleotide).count();

    Result::Ok(amount)
}
