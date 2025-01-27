
pub fn max_sub_array(nums: Vec<i32>) -> i32 {

    if nums.len() == 0 {
        return 0;
    }
    let mut max  = nums[0];

    let mut pre_sum = 0;

    nums.iter().for_each(|x| {
       if pre_sum > 0 {
          pre_sum += x;
       } else {
          pre_sum = *x;
       }
       max = max.max(pre_sum);
   });
    max
}
