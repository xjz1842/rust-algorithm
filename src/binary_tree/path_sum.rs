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
use std::collections::HashMap;
use std::path::Prefix;
use std::rc::Rc;
use std::cell::RefCell;

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, 
    target_sum: i32) -> i32 {
    let mut prefix = HashMap::new();
    prefix.insert(0, 1);
    let mut result = 0;
    dfs(&root,0,target_sum as i64,&mut result, &mut prefix);
    result
}

fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, sum: i64,
    target_sum: i64, result:&mut i32,
  prefix: &mut HashMap<i64, i32>)  {

    if let Some(node) = root {
        let n = node.borrow_mut();

        let sum= sum + n.val as i64;
        *result += *prefix.get(&(sum - target_sum)).unwrap_or(&0);
      
        *prefix.entry(sum).or_insert(0) += 1;
        dfs(&n.left, sum, target_sum, result, prefix);
        dfs(&n.right,sum, target_sum, result, prefix);
        *prefix.entry(sum).or_insert(0) -= 1; 
    }
 }

#[test]
fn path_sum_test() {
    let mut root = Some(Rc::new(RefCell::new(
        TreeNode::new(1))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(3))));
    }
    let target_sum = 5;
    println!("{:?}",path_sum(root,target_sum));   
}