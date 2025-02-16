pub fn can_jump(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut  right_most = 0;

    for i in 0..len-1 {
        if right_most < i {
            return false;
        }
         right_most = right_most.max(i + nums[i] as usize);
    }
    right_most >= len-1
}

#[test]
fn can_jump_test() {
   let nums  = vec![0,2,3];
   assert_eq!(false,can_jump(nums));
}