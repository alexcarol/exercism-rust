pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)|
            row
                .iter()
                .enumerate()
                .filter_map(move |(column_index, _cell)|
                    if is_saddle_point(row_index, column_index, input) {
                        Some((row_index, column_index))
                    } else {
                        None
                    }
                )
        ).collect()
}

fn is_saddle_point(i: usize, j: usize, input: &[Vec<u64>]) -> bool {
    let element = input[i][j];

    input[i]
        .iter()
        .map(|value| element < *value)
        .filter(|element_less_than_value| *element_less_than_value)
        .count() <= 0
        && input
        .iter()
        .map(|row| element > row[j])
        .filter(|is_greater| *is_greater)
        .count() == 0
}