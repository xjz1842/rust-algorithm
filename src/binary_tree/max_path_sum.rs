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
use std::cell::RefCell;

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
     let mut result = i32::MIN;
     dfs(&root, &mut result);
     result
}

fn  dfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
    if let Some(node) = node {
      let node = node.borrow();
      let left_value = dfs(&node.left, result);
      let right_value = dfs(&node.right, result);
      
      *result = (*result).max(left_value + node.val + right_value);

      return 0.max(left_value.max(right_value)) + node.val;
    }
    0
}

#[test]
fn max_path_sum_test() {
    let mut root = Some(Rc::new(RefCell::new(
        TreeNode::new(1))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(3))));
    }
    println!("{:?}",max_path_sum(root));   
}