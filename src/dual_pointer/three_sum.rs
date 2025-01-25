

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let n = nums.len();
    let mut ans = vec![];
    
    for i in 0..n-2 {
        let x = nums[i];
        if i > 0 && x == nums[i - 1] {
            // 跳过重复数字
            continue;
        }
        if x + nums[i + 1] + nums[i + 2] > 0 {
            // 优化一
            break;
        }
        if x + nums[n - 2] + nums[n - 1] < 0 {
            // 优化二
            continue;
        }
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            let s = x + nums[j] + nums[k];
            if s > 0 {
                k -= 1;
            } else if s < 0 {
                j += 1;
            } else {
                // 三数之和为 0
                ans.push(vec![x, nums[j], nums[k]]);
                j += 1;
                while j < k && nums[j] == nums[j - 1] {
                    // 跳过重复数字
                    j += 1;
                }
                k -= 1;
                while k > j && nums[k] == nums[k + 1] {
                    // 跳过重复数字
                    k -= 1;
                }
            }
        }
    }
    ans
}
