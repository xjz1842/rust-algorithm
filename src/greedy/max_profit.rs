
pub fn max_profit(prices: Vec<i32>) -> i32 {
       let mut result = 0 ;

       let len = prices.len();
       // 记录i 从右到左的最大高度
       let mut right_max = vec![0;len];
       right_max[len-1] = prices[len - 1];
       for i in (0..len-1).rev() {
            right_max[i] = right_max[i+1].max(prices[i]);
       }
       for i in 0..len {
           result  = result.max(right_max[i] - prices[i]);
       }
       result
}

#[test]
fn max_profit_test(){
    let prices = vec![7,6,4,3,1];
    assert_eq!(0, max_profit(prices));
}