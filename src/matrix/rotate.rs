

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n: usize = matrix.len();
    
    for row in 0..n/2 {
        for col in row..(n - row - 1) {
            let swap = (
                 (row, col),
                 (col, n - row - 1),
                 (n - row - 1, n - col - 1),
                 (n - col - 1, row));
           // swap
           let tmp = matrix[swap.3.0][swap.3.1];
           matrix[swap.3.0][swap.3.1] =  matrix[swap.2.0][swap.2.1];
           matrix[swap.2.0][swap.2.1] =  matrix[swap.1.0][swap.1.1];
           matrix[swap.1.0][swap.1.1] =  matrix[swap.0.0][swap.0.1];
           matrix[swap.0.0][swap.0.1] = tmp;
        }
    }
}