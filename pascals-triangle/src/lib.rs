pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
		let mut rows = vec![];

        let mut previous_row = vec![];
        for i in 1..(row_count + 1) {
            let mut current_row = vec![1; i as usize];

            for j in 1..(current_row.len() - 1) {
                current_row[j] = previous_row[j -1] + previous_row[j]
            }

            previous_row = current_row.clone();

            rows.push(current_row);
        }
        PascalsTriangle {
            rows
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
