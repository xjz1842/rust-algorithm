

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0 ;
    let mut right = nums.len();

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

