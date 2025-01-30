

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // 从右上角出发
    let mut row = 0;
    let mut col = cols - 1;

     while row < rows &&  col < cols {
        if matrix[row][col] == target {
            return true;
        } else if  matrix[row][col] > target {
             //排除该列
             col -= 1;
        } else {
             //排除该行
            row += 1;
        }
     }  
     false
}