pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points =  vec![];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if is_saddle_point(i, j, input) {
                points.push((i, j));
            }
        }
    }

    points
}

fn is_saddle_point(i: usize, j:  usize, input: &[Vec<u64>]) -> bool {
    let element = input[i][j];

    for k in 0..input[i].len() {
        if element < input[i][k] {
            return false
        }
    }

    for k in 0..input.len() {
        if element > input[k][j] {
            return false
        }
    }

    true
}