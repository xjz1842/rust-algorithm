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

pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>,
     p: Option<Rc<RefCell<TreeNode>>>, 
     q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
     if root.is_none() || root == p  || root == q {
        return root;
     }
     let x = root.as_ref()?;
     
     let left = lowest_common_ancestor(
        x.borrow_mut().left.take(), p.clone(), q.clone());
     let right = lowest_common_ancestor(
            x.borrow_mut().right.take(), p, q);

     if left.is_some() && right.is_some() {
        return root;
     }       
     if left.is_some(){
        left
     } else {
        right
     }    
}


#[test]
fn lowest_common_ancestor_test() {
    let mut root = Some(Rc::new(RefCell::new(
        TreeNode::new(1))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(3))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
    }

    let p = Some(Rc::new(RefCell::new(
        TreeNode::new(2))));
    let q= Some(Rc::new(RefCell::new(
        TreeNode::new(3))));

     assert_eq!(root, lowest_common_ancestor(root.clone(), p, q));
}