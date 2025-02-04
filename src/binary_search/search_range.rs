
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let start =  lower(&nums, target);

    if start == nums.len() as i32 || nums[start as usize] != target {
       return vec![-1,-1];
    }
    let end =  lower(&nums, target + 1) - 1;
     vec![start,end]
}

pub fn lower(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        //开区间
        while left < right {
            let mid = left + (right - left) / 2;
             if nums[mid] < target {
                left = mid + 1;
             } else {
                right = mid;
             }
        }
        left as i32
 }
