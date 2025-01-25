use std::vec;

mod group_anagrams;
mod max_coins;
mod two_sum;
mod longest_consecutive;
mod move_zeroes;
mod max_area;
mod three_sum;

fn main() {
    let piles = vec![2, 4, 1, 2, 7, 8];
    println!("{}", max_coins::max_coins(piles));

    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("{:?}", two_sum::two_sum1(nums, target));

    let strs :Vec<&str> = vec!["eat", "tea", "tan",
     "ate", "nat", "bat"];
    println!("{:?} ", group_anagrams::
    group_anagrams(strs.iter().map(|x|x.to_string()).collect()));

    let nums = vec![0,3,7,2,5,8,4,6,0,1];
    println!("{}", longest_consecutive::longest_consecutive(nums));

    let mut nums  = vec![0,1,0,3,12];
    move_zeroes::move_zeroes(&mut nums);
    println!(" {:?} ", nums);


    let height = vec![1,8,6,2,5,4,8,3,7];
    println!(" {} ", max_area::max_area(height));


    let  nums= vec![-1,0,1,2,-1,-4];
    println!(" {:?} ", three_sum::three_sum(nums));
}
