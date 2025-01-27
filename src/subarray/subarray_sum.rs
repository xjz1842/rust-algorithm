use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;

    let len = nums.len();
    // count preSum
    let mut pre_sum = vec![0;len];

    for (i,v) in nums.iter().enumerate() {
        if i == 0 {
            pre_sum[i] = *v;
        } else {
            pre_sum[i] = pre_sum[i - 1] + v;
        }
    }

    for i in 0..len {
        for j in i..len {
            if pre_sum[j] - pre_sum[i] + nums[i] == k {
                result  += 1;
            }
        }
    }
    result
}

pub fn subarray_sum1(nums: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;

    let len = nums.len();
    // count preSum
    let mut pre_sum = 0;
    let mut map = HashMap::new();
    // sum = 0 
    map.insert(0,1);

    for i in 0..len {
        pre_sum += nums[i];

        let target = pre_sum - k;

        if map.contains_key(&target) {
            if let Some(v) = map.get(&target) {
                result += v;
            }
        }
        let v: &mut i32 = map.entry(pre_sum).or_insert(0);
        *v += 1;
    }

    result
}