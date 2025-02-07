
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    // 0..i 个元素最长严格递增子序列的长度 为 dp[i]
    let mut  dp = vec![1;len];
    let mut max = 0;
    for i in 1..len {
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            } 
            max = max.max(dp[i]);
        }
    }
   
}

#[test]
fn length_of_lis_test() {
 let nums =  vec![1,3,6,7,9,4,10,5,6];
 assert_eq!(6,length_of_lis(nums));
}
