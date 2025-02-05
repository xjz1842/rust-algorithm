

pub fn num_squares(n: i32) -> i32 {
    if n == 1{
        return 1;
    }
    //[i]表示i最少完全平方数的个数
    let mut  dp = vec![i32::MAX; (n+1) as usize];
    //初始化
    dp[0] = 0;
    dp[1] = 1;
    for i in 1..=n{
       let mut k = 1;
       let mut s =  k * k;
       while s <= i {
           dp[i as usize] = (dp[(i-s) as usize] + 1).min(dp[i as usize]);
           k += 1;
           s = k * k;
       }
    }
   return dp[n as usize];
}


#[test] 
fn num_squares_test() {
    let n = 13;
    assert_eq!(2,num_squares(n));
}