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

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut head: Option<Rc<RefCell<TreeNode>>> = None;
    dfs(root, &mut head);
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, 
    head: &mut Option<Rc<RefCell<TreeNode>>>) {
    if let Some(x) = node {
        let mut x = x.borrow_mut();
        dfs(&x.right, head);
        dfs(&x.left, head);
        x.left = None;
        x.right = head.take(); 
        *head = node.clone();
    }
}

#[test]
fn flatten_test() {
    let mut root = Some(Rc::new(RefCell::new(
        TreeNode::new(1))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(3))));
    }
    flatten(&mut root);
    println!("{:?}",root);
}
