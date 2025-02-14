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

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
     let mut inorder_list = Vec::new();
     traversal(&root,&mut inorder_list);
     inorder_list
}

fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, inorder_list : &mut Vec<i32>) {
  
  match root {
    Some(node) => {
      traversal(&node.borrow().left, inorder_list);
      inorder_list.push(node.borrow().val);
      traversal(&node.borrow().right, inorder_list);
    },
    None => return,
  }
}



#[test]
fn inorder_traversal_test() {
    let mut root = Some(Rc::new(RefCell::new(
        TreeNode::new(1))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(3))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
    }
    println!("{:?}", inorder_traversal(root));
}

