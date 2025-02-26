pub fn unique_paths(m: i32, n: i32) -> i32 {
    //代表i,j位置的多少条路径
    let mut dp = vec![vec![0;n as usize];m as usize];
   
    // 第一行
    for c in 0..n as usize {
       dp[0][c] = 1;
    }
    // 第一列
    for r in 0..m as usize{
        dp[r][0] = 1;
     }

    for i in 1..m as usize {
        for j in 1..n as usize{
            dp[i][j] = dp[i-1][j] + dp[i][j-1];
        }
    }
    dp[m as usize-1][n as usize-1]
}


#[test]
fn unique_paths_test() {
  let m = 3;
  let n = 7;
  assert_eq!(28,unique_paths(m, n));
}