

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow:i32 = 0;
        let mut fast:i32 = 0;

        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }
        slow = 0;
        while slow != fast{
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }
        slow as i32
}