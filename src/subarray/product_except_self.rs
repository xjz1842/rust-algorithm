

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
       let len  = nums.len();
        let mut result = vec![0;len];
        let mut pre_multi : Vec<i32> = vec![0;len];
        // 表示i左边的所有数的乘积
        pre_multi[0] = 1;
        for i in 1..len {
            // preMulti[i - 1] 表示 i - 2 前面的
            pre_multi[i] =  pre_multi[i - 1] as i32  * nums[i-1];
        }
        let mut r_mut = 1;
        for i in (0..len).rev(){
            result[i] = pre_multi[i] * r_mut;
            r_mut *= nums[i];
        }
        result
}


#[test]
fn product_except_self_test() {
    let nums = vec![1,2,3,4];
    let result = product_except_self(nums);
    assert_eq!(result, vec![24,12,8,6]);
}