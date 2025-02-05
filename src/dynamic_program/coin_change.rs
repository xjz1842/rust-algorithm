pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }
    // 0..i 硬币凑成总金额 dp[i] 的最少的硬币个数
    let mut dp = vec![0; (amount + 1) as usize];
    // init
    dp[0] = 0;
    for coin in coins {
        if coin <= amount {
            dp[coin as usize] = 1;
        }
        for i in coin..=amount {
            let mut count = 1;
            while coin * count <= i {
                if dp[(i - coin * count) as usize] > 0 {  
                    if dp[i as usize] > 0 {
                        //取count个元素
                        dp[i as usize] = dp[i as usize].min(dp[(i - coin * count) as usize] + count);
                    } else {
                        dp[i as usize] = dp[(i - coin * count) as usize] + count;
                    }
                }
                count += 1;
            }
        }
    }
    let result = if dp[amount as usize] == 0 {
        -1
    } else {
        dp[amount as usize]
    };
    result
}
