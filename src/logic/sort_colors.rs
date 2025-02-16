pub fn sort_colors(nums: &mut Vec<i32>) {
     // 先将0移动左边
    let mut left = 0;
    let mut idx = 0;
    let len = nums.len(); 
    let mut right = len - 1;
    while idx <= right { 
        if nums[idx] == 0 && left != idx {
            nums.swap(left, idx);
            left += 1;    
        } else if nums[idx] == 2 && right != idx{
            nums.swap(right, idx);
            right -= 1;
        } else {
            idx += 1;
        }
    }
}

#[test]
fn sort_colors_test() {
    let mut nums = vec![2];
    sort_colors(&mut nums);
    println!("{:?}", nums);
    assert_eq!(vec![2],nums);
}