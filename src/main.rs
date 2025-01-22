mod max_coins;
mod two_sum;

fn main() {
 let piles= vec![2,4,1,2,7,8];
 println!("{}", max_coins::max_coins(piles));

 let nums= vec![2,7,11,15];
 let target = 9;
 println!("{:?}",two_sum::two_sum1(nums,target));
}
