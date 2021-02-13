pub struct PascalsTriangle {
    pub row: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut row_content: Vec<Vec<u32>> = Vec::new();
        for i in 0..row_count {
            let mut inner_row: Vec<u32> = Vec::new();
            for j in 0..(i+1) {
                inner_row.push(binomial_coeff(i, j))
            } 
            row_content.push(inner_row);
        } 

        PascalsTriangle {
            row: row_content
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        return self.row.clone()
    }
}

fn binomial_coeff(n: u32, mut k: u32) -> u32 {
    let mut r: u32 = 1;
    if k > n - k {
        k = n - k; 
    }
    for i in 0..k {
        r = r * (n - i); 
        r = r / (i + 1);
    }
    return r
}
     