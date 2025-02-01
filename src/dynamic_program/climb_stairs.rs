


pub fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![0; (n+1) as usize];
    if n == 1 {
        return 1;
    }
    if n == 2  {
        return 2;
    } 
    //初始化
    dp[1] = 1;
    dp[2] = 2;
     for i in 3..=n  as usize{
       dp[i] =  dp[i-1] + dp[i-2];
    }
    dp[n as usize]
}