
/**
 * 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
   请注意 ，必须在不复制数组的情况下原地对数组进行操作。
*/
pub fn move_zeroes(nums: &mut Vec<i32>) {
      // 双指针
      let mut left = 0;
      let mut i = 0 ;
      let right = nums.len() -1 ;

      if nums.len() <= 1 {
         return;
      }
      while i <= right {
        // find left zero
        if nums[i] != 0  {
           nums.swap(left,i);
           left += 1;
        } 
        i += 1;
      } 
}