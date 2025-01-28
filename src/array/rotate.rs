

pub fn rotate(nums: &mut Vec<i32>, k: i32) {

    let len = nums.len();

    let k = k % (len as i32);
    // reverse
    nums.reverse();
 
    // reverse k
    nums[0..(k as usize)].reverse();

 
    // reverse n - k 
    nums[(k as usize)..len].reverse();
}
