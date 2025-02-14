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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  return process(&root,&root);
}

fn process(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>)->bool {
    if *left == None && *right == None {
        return true;
    }
   
    match (left, right) {
      (Some(left), Some(right)) => {
          let left = left.borrow();
          let right = right.borrow();
          left.val == right.val && process(&left.left, &right.right) && 
          process(&left.right, &right.left)
      }
      (None, None) => true,
      _ => false,
  }
}


#[test]
fn is_symmetric_test() {
    let mut root = Some(Rc::new(RefCell::new(
        TreeNode::new(1))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
    }
    assert_eq!(true, is_symmetric(root));
}