



pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    for i in 0..len {
        while nums[i] <= len as i32 && nums[i] > 0
         &&  nums[(nums[i] - 1) as usize] != nums[i] {
            let j: i32 = nums[i] - 1;
            nums.swap(j as usize,i  );
        }
    }
    for i in 0..len {
        if ((nums[i]) as usize) !=  (i+1) {
            return (i + 1) as i32;
        }
    }
    (len + 1) as i32
}