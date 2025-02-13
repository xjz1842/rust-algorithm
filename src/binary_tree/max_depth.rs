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

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
     if root.is_none() {
        return 0;
     }else {
        let node = root.unwrap();
        let node = node.borrow();
         return (max_depth(node.left.clone()) + 1).max(
            max_depth(node.right.clone())+1);
     }
}




#[test]
fn max_depth_test() {
       // tree
       let root = Some(Rc::new(RefCell::new(
        TreeNode::new(1),
    )));
    println!("{}",max_depth(root))
}

