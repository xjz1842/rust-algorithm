

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let len  = nums.len();
        let mut pre_multi : Vec<i32> = vec![0;len];
        let mut suffix_multi : Vec<i32> = vec![0;len];
        pre_multi[0] = 1;
        for i in 1..len {
            // preMulti[i - 1] 表示 i - 2 前面的
            pre_multi[i] =  pre_multi[i - 1] as i32  * nums[i-1];
        }
        suffix_multi[len-1] = 1;

        for i in (0..len-1).rev() {
            suffix_multi[i] =  suffix_multi[i + 1] as i32  * nums[i + 1];
        }
        for i in 0..len {
            result.push(pre_multi[i] * suffix_multi[i]);
        }
        result
}