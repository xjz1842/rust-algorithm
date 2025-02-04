use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

mod array;
mod binary_search;
mod binary_tree;
mod dual_pointer;
mod greedy;
mod hash;
mod matrix;
mod slide_window;
mod subarray;

mod linklist;

mod stack;

mod backstrace;

mod dynamic_program;

mod graph;

mod logic;

use dual_pointer::max_area;
use dual_pointer::move_zeroes;
use dual_pointer::three_sum;
use dual_pointer::trap;
use greedy::max_coins;
use hash::group_anagrams;
use hash::longest_consecutive;
use hash::two_sum;
use linklist::reverse_list::ListNode;
use slide_window::find_anagrams;
use slide_window::length_of_longest_substring;
use subarray::max_sliding_window;
use subarray::min_window;
use subarray::product_except_self;
use subarray::subarray_sum;

use array::first_missing_positive;
use array::max_sub_array;
use array::merge;
use array::merge_k;
use array::rotate;

use binary_search::search_insert;
use binary_search::search_matrix as search_matrix_binary_search;

use matrix::rotate as matrix_rotate;
use matrix::search_matrix;
use matrix::set_zeroes;
use matrix::spiral_order;

use linklist::reverse_list;
use linklist::is_palindrome;

use stack::daily_temperatures;

use binary_tree::diameter_of_binary_tree;

use backstrace::combination_sum;
use backstrace::generate_parenthesis;
use backstrace::letter_combinations;
use backstrace::permute;
use backstrace::subsets;

use dynamic_program::climb_stairs;
use dynamic_program::rob;

use graph::num_islands;

use logic::find_duplicate;

fn main() {
    let piles = vec![2, 4, 1, 2, 7, 8];
    println!("{}", max_coins::max_coins(piles));

    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("{:?}", two_sum::two_sum1(nums, target));

    let strs: Vec<&str> = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
    println!(
        "{:?} ",
        group_anagrams::group_anagrams(strs.iter().map(|x| x.to_string()).collect())
    );

    let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    println!("{}", longest_consecutive::longest_consecutive(nums));

    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes::move_zeroes(&mut nums);
    println!(" {:?} ", nums);

    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!(" {} ", max_area::max_area(height));

    let nums = vec![-1, 0, 1, 2, -1, -4];
    println!(" {:?} ", three_sum::three_sum(nums));

    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!(" {:?} ", trap::trap(height));

    let s = "abcabcbb".to_string();
    println!(
        "{}",
        length_of_longest_substring::length_of_longest_substring(s)
    );

    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();
    println!("{:?}", find_anagrams::find_anagrams(s, p));

    let nums = vec![1, 2, 1, 2, 1];
    let k = 3;
    println!("{}", subarray_sum::subarray_sum(nums.clone(), k));
    println!("{}", subarray_sum::subarray_sum1(nums.clone(), k));

    let nums = vec![5, 4, -1, 7, 8];
    println!("{}", max_sub_array::max_sub_array(nums));

    let nums = vec![1, 3, 5, 6];
    let target = 2;
    println!(
        "search_insert {}",
        search_insert::search_insert(nums, target)
    );

    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 3;
    println!(
        "search_matrix_binary_search::
    search_matrix {}",
        search_matrix_binary_search::search_matrix(matrix, target)
    );

    let intervals = vec![vec![1, 4], vec![2, 3]];
    println!(" {:?} ", merge::merge(intervals));

    let intervals = vec![vec![1, 4], vec![2, 3]];
    println!(
        " merge_k_sorted_vecs {:?} ",
        merge_k::merge_k_sorted_vecs(intervals)
    );

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    rotate::rotate(&mut nums, k);
    println!("{:?}", nums);

    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    println!("{:?}", max_sliding_window::max_sliding_window(nums, k));

    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    println!("{:?}", min_window::min_window(s, t));

    let nums = vec![-1, 1, 0, -3, 3];
    println!("{:?}", product_except_self::product_except_self(nums));

    let nums = vec![-1, 4, 2, 1, 9, 10];
    println!("{:?}", first_missing_positive::first_missing_positive(nums));

    let mut matrix: Vec<Vec<i32>> = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    set_zeroes::set_zeroes(&mut matrix);
    println!("{:?}", matrix);

    let matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{:?}", spiral_order::spiral_order(matrix));

    let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    matrix_rotate::rotate(&mut matrix);
    println!("{:?}", matrix);

    let matrix: Vec<Vec<i32>> = vec![vec![-5]];
    let target = 5;
    println!("{:?}", search_matrix::search_matrix(matrix, target));

    let mut head = reverse_list::ListNode::new(1);
    head.next = Some(Box::new(ListNode::new(2)));
    println!("{:?}", reverse_list::reverse_list(Some(Box::new(head))));
   
    let mut head = is_palindrome::ListNode::new(1);
    head.next = Some(Box::new(is_palindrome::ListNode::new(2)));
    println!("is_palindrome {:?}", is_palindrome::is_palindrome(Some(Box::new(head))));

    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    println!("{:?}", daily_temperatures::daily_temperatures(temperatures));

    let nums = vec![1, 2, 3];
    println!("{:?}", permute::permute(nums));
    let nums = vec![1, 2, 3];
    println!("subsets {:?}", subsets::subsets(nums));
    let n = 3;
    println!(
        "generate_parenthesis  {:?}",
        generate_parenthesis::generate_parenthesis(n)
    );
    let digits = "23".to_string();
    println!(
        "letter_combinations {:?}",
        letter_combinations::letter_combinations(digits)
    );
    let candidates = vec![7, 3, 2];
    let target = 18;
    println!(
        "combination_sum {:?}",
        combination_sum::combination_sum(candidates, target)
    );

    let n = 10;
    println!(" {} ", climb_stairs::climb_stairs(n));

    let nums = vec![1, 2, 3, 1];
    println!("rob {}", rob::rob(nums));

    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];

    println!("nums of islands  {} ", num_islands::num_islands(grid));

    let nums = vec![1, 3, 4, 2, 2];
    println!("{}", find_duplicate::find_duplicate(nums));

    // tree
    let root = Some(Rc::new(RefCell::new(
        diameter_of_binary_tree::TreeNode::new(1),
    )));
    println!("{}", diameter_of_binary_tree::diameter_of_binary_tree(root));
}
