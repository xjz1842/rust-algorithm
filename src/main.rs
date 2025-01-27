use std::vec;

mod dual_pointer;
mod hash;
mod greedy;
mod slide_window;
mod subarray;
mod array;
mod binary_search;

use hash::group_anagrams;
use hash::longest_consecutive;
use hash::two_sum;
use greedy::max_coins;
use dual_pointer::max_area;
use dual_pointer::move_zeroes;
use dual_pointer::three_sum;
use dual_pointer::trap;
use slide_window::length_of_longest_substring;
use slide_window::find_anagrams;
use subarray::subarray_sum;
use array::max_sub_array;
use array::merge;

use binary_search::search_insert;

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

    let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    println!(" {:?} ",    trap::trap(height));

    let s = "abcabcbb".to_string();
    println!("{}",length_of_longest_substring::
    length_of_longest_substring(s));

    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();
    println!("{:?}",find_anagrams::find_anagrams(s,p));

    let nums = vec![1,2,1,2,1];
    let k = 3;
    println!("{}", subarray_sum::subarray_sum(nums.clone(), k));
    println!("{}", subarray_sum::subarray_sum1(nums.clone(), k));

    let nums = vec![5,4,-1,7,8];
    println!("{}",max_sub_array::max_sub_array(nums));

    let nums = vec![1,3,5,6];
    let target = 0;
    println!("{}", search_insert::search_insert(nums, target));

   let intervals = vec![vec![1,4],vec![2,3]];
   println!(" {:?} ", merge::merge(intervals))
}
