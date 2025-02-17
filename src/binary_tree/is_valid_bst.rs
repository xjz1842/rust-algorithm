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

 pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
       valid(&root,i64::MIN, i64::MAX)
 }

 fn valid(root: &Option<Rc<RefCell<TreeNode>>>, low : i64, upper : i64) -> bool {
      
      match root {
         Some(node) => {
           let borrow_node =   node.borrow();
           if borrow_node.val as i64 >= upper
             || borrow_node.val as i64 <= low {
                return false;
             }
             return valid(&borrow_node.left, low, borrow_node.val as i64)
               && valid(&borrow_node.right, borrow_node.val as i64, upper);
             },
            None => true
        }
 }



#[test]
fn is_valid_bst_test() {
    let mut root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(
        TreeNode::new(2147483647))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(1))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(3))));
    }
    println!("{:?}", is_valid_bst(root));
}