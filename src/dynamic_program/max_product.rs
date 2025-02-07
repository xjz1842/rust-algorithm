

pub fn max_product(nums: Vec<i32>) -> i32 {
      let mut f_max = vec![0; n];
      let mut f_min = vec![0; n];

      f_max[0] = nums[0];
      f_min[0] = nums[0];

      for i in 1..nums.len() {
        let x = nums[i];
        // 把 x 加到右端点为 i-1 的（乘积最大/最小）子数组后面，
        // 或者单独组成一个子数组，只有 x 一个元素
         f_max[i] = x.max(f_max[i - 1] * x).max(f_min[i - 1] * x);
         f_min[i] = x.min(f_max[i - 1] * x).min(f_min[i - 1] * x);
      }
      *f_max.iter().max().unwrap()
}


#[test]
fn max_product_test() {
    let nums = vec![-2,0,-1];
    assert_eq!(0, max_product(nums));
}