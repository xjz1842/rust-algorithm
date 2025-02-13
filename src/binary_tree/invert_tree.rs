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

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let left = invert_tree(node.borrow_mut().left.take());
        let right = invert_tree(node.borrow_mut().right.take());
        node.borrow_mut().left = right;
        node.borrow_mut().right = left;
        Some(node)
    } else {
       None
    }
}

#[test]
fn invert_tree_test() {
    let mut root = Some(Rc::new(RefCell::new(
        TreeNode::new(1))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(3))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
    }
    println!("{:?}", invert_tree(root));
}
