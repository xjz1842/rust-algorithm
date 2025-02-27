// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell:: RefCell;

pub fn sorted_array_to_bst(nums: Vec<i32>) ->
 Option<Rc<RefCell<TreeNode>>> {
    return build(&nums, 0, nums.len() as i32);
}

fn build(nums: &Vec<i32>,left: i32, right:i32) ->
Option<Rc<RefCell<TreeNode>>> {
    if left >= right { return None; }
    let mid = (left + right) / 2;
    let mut node = TreeNode::new(nums[mid as usize]);

    node.left = build(nums, left , mid);
    node.right = build(nums, mid + 1, right);

    return Some(Rc::new(RefCell::new(node)));
}

#[test]
fn sorted_array_to_bst_test() {
     // tree
     let nums = vec![-10,-3,0,5,9];
    println!("{:?}",sorted_array_to_bst(nums))
}

