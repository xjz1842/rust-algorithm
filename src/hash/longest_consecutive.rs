use std::collections::HashSet;

/**
 * 给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。

   请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
   0 <= nums.length <= 105
   -109 <= nums[i] <= 109
 */
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut res = 0;

    if nums.len() == 0 {
        return res;
    }
    let nset: HashSet<i32> = nums.into_iter().collect();
   
    let mut max_len: i32 = 1;
    for i in &nset {
       if !nset.contains(&(i-1)) {
              let mut left= *i;
              while nset.contains(&(left + 1)){
                    left += 1;
                    max_len += 1;
              }
              //result 
              res = res.max(max_len);
              max_len = 1;
        }
    }
    res
}