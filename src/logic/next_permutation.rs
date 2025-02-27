



pub fn next_permutation(nums: &mut Vec<i32>) {
    if nums.len() == 1 {
        return;
    }
    let mut  i = nums.len() -1;
    // 从后往前寻找降序对
    while i > 0 {
        if nums[i-1] < nums[i] {
             break;
        }
        i -= 1;
    }
    if i == 0 {
        nums.reverse();
        return;
    }
    let len = nums.len();
    // 找一个[i, end] 找一个比 i-1 大的交换
    for j in (i..len).rev() {
       if nums[i-1] < nums[j] {
         nums.swap(i-1, j);
         // 逆序[i，end]
         nums[i..len].reverse();
         break;
     }
   }

}

#[test]
fn next_permutation_test() {
   let mut nums = vec![5,4,7,5,3,2];
   next_permutation(&mut nums);
   println!("{:?}", nums);
}