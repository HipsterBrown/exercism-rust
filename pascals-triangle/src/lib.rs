pub struct PascalsTriangle {
    count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut triangle: Vec<Vec<u32>> = Vec::with_capacity(self.count as usize);
        for row_num in 0..self.count {
            triangle.push(
                (0..row_num+1).map(|col_num| {
                    if col_num == 0 || col_num == row_num {
                        return 1u32;
                    }
                    factorial(row_num) / (factorial(col_num) * factorial(row_num - col_num))
                }
            ).collect());
        }
        triangle
    }
}

fn factorial(base: u32) -> u32 {
    (1..base+1).product()
}
