mod group_anagrams;
mod max_coins;
mod two_sum;

fn main() {
    let piles = vec![2, 4, 1, 2, 7, 8];
    println!("{}", max_coins::max_coins(piles));

    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("{:?}", two_sum::two_sum1(nums, target));

    let strs :Vec<&str> = vec!["eat", "tea", "tan",
     "ate", "nat", "bat"];
    print!("{:?} ", group_anagrams::
    group_anagrams(strs.iter().map(|x|x.to_string()).collect()));
}
