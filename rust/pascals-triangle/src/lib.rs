pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let rows = (1..=row_count)
            .map(|j| {
                let mut row = Vec::with_capacity(j as usize);
                row.push(1);
                for i in 1..j {
                    row.push(row[i as usize - 1] * (j - i) / i);
                }
                row
            })
            .collect();
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
