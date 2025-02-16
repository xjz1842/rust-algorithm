

pub fn jump(nums: Vec<i32>) -> i32 {
     let mut jump = 0;
     let len = nums.len();
     let mut i: usize = 0;
     let mut max_position = 0;
     let mut end = 0;

     while i < len - 1{
        max_position = max_position.max( i + nums[i] as usize);
       if end == i {
           end = max_position;
           jump += 1;
       } 
        i += 1;
     } 
     jump
}

#[test]
fn jump_test() {
    let nums  = vec![2,3,1,1,4];
    println!(" {} ",jump(nums));
}