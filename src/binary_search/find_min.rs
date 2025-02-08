pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let mid: usize = left + (right - left) / 2;
        if nums[mid] < nums[nums.len()-1]  {
              right = mid;
        } else {
             left = mid + 1;
        }
    }
    nums[left]
}

#[test]
fn find_min_test() {
    let nums = vec![4,5,6,7,0,1,2];
    assert_eq!(0,find_min(nums));
}
