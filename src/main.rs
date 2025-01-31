use std::vec;

mod dual_pointer;
mod hash;
mod greedy;
mod slide_window;
mod subarray;
mod array;
mod binary_search;
mod binary_tree;
mod matrix;

mod linklist;

mod stack;

use hash::group_anagrams;
use hash::longest_consecutive;
use hash::two_sum;
use greedy::max_coins;
use dual_pointer::max_area;
use dual_pointer::move_zeroes;
use dual_pointer::three_sum;
use dual_pointer::trap;
use linklist::reverse_list::ListNode;
use slide_window::length_of_longest_substring;
use slide_window::find_anagrams;
use subarray::subarray_sum;
use subarray::max_sliding_window;
use subarray::min_window;
use subarray::product_except_self;

use array::max_sub_array;
use array::merge;
use array::rotate;
use array::first_missing_positive;

use binary_search::search_insert;

use matrix::set_zeroes;
use matrix::spiral_order;
use matrix::rotate as matrix_rotate;
use matrix::search_matrix;

use linklist::reverse_list;

use stack::daily_temperatures;

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
   println!(" {:?} ", merge::merge(intervals));

   let mut nums = vec![1,2,3,4,5,6,7];
   let k = 3;
   rotate::rotate(& mut nums, k);
   println!("{:?}",nums);

   let nums = vec![1,3,-1,-3,5,3,6,7];
   let k = 3;
   println!("{:?}", max_sliding_window::max_sliding_window(nums, k));
     
    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    println!("{:?}", min_window::min_window(s, t));

    let nums = vec![-1,1,0,-3,3];
    println!("{:?}", product_except_self::product_except_self(nums));

    let nums = vec![-1,4,2,1,9,10];
    println!("{:?}",first_missing_positive::first_missing_positive(nums));

    let mut matrix:Vec<Vec<i32>> = vec![vec![0,1,2,0],vec![3,4,5,2],
    vec![1,3,1,5]];
    set_zeroes::set_zeroes(&mut matrix);
    println!("{:?}",matrix);

    let matrix:Vec<Vec<i32>> = vec![vec![1,2,3],vec![4,5,6],
    vec![7,8,9]];
    println!("{:?}",spiral_order::spiral_order(matrix));

    let mut matrix:Vec<Vec<i32>> = vec![vec![1,2,3],vec![4,5,6],
    vec![7,8,9]];
    matrix_rotate::rotate(&mut matrix);
    println!("{:?}", matrix);

     let matrix: Vec<Vec<i32>> =
      vec![vec![-5]];
      let target = 5;
      println!("{:?}", search_matrix::search_matrix(matrix, target));
         
     let mut head =  reverse_list::ListNode::new(1);
     head.next = Some(Box::new(ListNode::new(2)));
     println!("{:?}", reverse_list::reverse_list(Some(Box::new(head))));

    let temperatures = vec![73,74,75,71,69,72,76,73];
    println!("{:?}", daily_temperatures::daily_temperatures(temperatures));

}
