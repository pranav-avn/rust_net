pub struct Matrix{
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix{
    pub fn zeros(rows: usize, cols:usize) -> Matrix{
        Matrix{
            rows,
            cols,
            data: vec![vec![0.0, cols], rows],
        }
    }
}