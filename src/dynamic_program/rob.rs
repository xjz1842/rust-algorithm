

pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![0;len + 1];
        // init 
        dp[0] = 0;
        if len == 1 {
            return nums[0];
        }
        dp[1] = nums[0];
        for i in 2..=len {
          dp[i] = dp[i-1].max(dp[i-2] + nums[i-1]);
        }
        dp[len]
}