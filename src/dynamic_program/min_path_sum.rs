pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
   let row = grid.len();
   let col = grid[0].len();
   // i j 位置的最小dp[i][j]
   let mut dp = vec![vec![0;col];row];
   dp[0][0] = grid[0][0];
   // init first row
   for c in 1..col {
      dp[0][c] = grid[0][c] + dp[0][c-1];
   }
    // init first col
   for r in 1..row {
      dp[r][0] += grid[r][0] + dp[r-1][0];
   }
   for r in 1..row {
     for c in 1..col {
        dp[r][c] = dp[r-1][c].min(dp[r][c-1]) + grid[r][c];
     }
   } 

   dp[row-1][col-1]
}

#[test]
fn min_path_sum_test() {
    let grid = vec![vec![1,3,1],
    vec![1,5,1],vec![4,2,1]];
    println!("{}", min_path_sum(grid));
}