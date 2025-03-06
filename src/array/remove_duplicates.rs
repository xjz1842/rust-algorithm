
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
       let mut left = 0;
       let mut i = 0;
       let len = nums.len();

       if len == 1 {
          return 1;
       }
       while i + 1 < len {
          while i + 1 < len && nums[i] == nums[i+1] {
               i += 1;
          } 
          nums[left] = nums[i];
          left += 1;
          i += 1;
       }

       if nums[len-1] != nums[left-1] {
           nums[left] = nums[len-1];
           left += 1;
       }
       left as i32
}

#[test]
fn remove_duplicates_test() {
    let mut nums = vec![0,0,1,1,1,2,2,3,3];
    let result = remove_duplicates(&mut nums);
    println!("{:?} {} ", nums, result);
}