

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
     let mut left: i32 = 0 ;
     let mut right: i32 = nums.len() as i32 - 1;

     while left <= right {
        let mid: i32 = left + (right - left) / 2; 

        if nums[mid as usize ] == target {
            return mid as i32;
        } else if nums[left as usize] <= nums[mid as usize] {
            // 左边有序，排除掉左边
             if nums[left as usize] <= target 
                 && target < nums[mid as usize] {
                    right = mid - 1;
             }  else {
                    left = mid + 1;
             }
        } else {
            // 右边边有序，排除掉右边
            if nums[mid as usize ] < target 
              && target <= nums[right as usize] {
                 left = mid + 1;
            } else {
                 right = mid - 1;
            }
        }
     }       
     -1
}

#[test]
fn search_test() {
    let nums = vec![3,1];
    assert_eq!(1,search(nums, 1));
}