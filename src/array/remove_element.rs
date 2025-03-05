pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.len() == 0 {
       return 0;
    }
    let mut left = 0 as i32;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        // 从左到右找到一个等于value的元素
        while left <= right && nums[left as usize] != val {
            left += 1;
        }
        // 从右到左找到一个不等于value的元素
        while left <= right && nums[right as usize] == val {
            right -= 1;
        }
        
        if right >= 0 && left < right{
            nums.swap(left as usize, 
                right as usize);
        }
    }    
    left
}

#[test]
fn remove_element_test() {
    let mut nums = vec![1];
    let val = 1;
    let r= remove_element(&mut nums, val);
    println!("{:?} {}",nums, r);
}