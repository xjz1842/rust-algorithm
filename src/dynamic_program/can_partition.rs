


pub fn can_partition(nums: Vec<i32>) -> bool {
    let mut sum = nums.iter().map(|x|*x as usize).sum::<usize>();

     if (sum & 1) == 1 {
        return  false;
     }
     sum = sum / 2;
     // 表示[0,i]元素中是否可以凑出 dp[i]
     let mut dp= vec![false;
      (sum + 1) as usize];

      dp[0] = true;
      let mut min: usize = 0;
      for num in nums.iter() {
        min = sum.min(min + *num as usize);
        for i in (*num as usize ..=sum).rev() {
           dp[i] = dp[i] || dp[i - *num as usize];
       }
     }
     dp[sum as usize]
} 