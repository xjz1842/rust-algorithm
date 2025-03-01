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
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
     let mut result = vec![];
      // BFS 
      let mut  queue = VecDeque::new();
      match root {
        Some(node) => {
            // head
            queue.push_back(node);
            // BFS
            while !queue.is_empty() {
                let len = queue.len();
                for i in  0..len{
                    let front = queue.pop_front();
                    if let  Some(node) = front {
                        if i == (len-1) {
                            result.push(node.borrow().val);
                        }
                        let mut b_node=  node.borrow_mut();
                        if let Some(n) = b_node.left.take() {
                            queue.push_back(n); 
                        }
                        if let Some(n) = b_node.right.take() {
                            queue.push_back(n); 
                        }
                    }
                   
                }
            }
        },
        None => () 
      }        
      result
}

#[test]
fn right_side_view_test() {
    let mut root = Some(Rc::new(RefCell::new(
        TreeNode::new(1))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(3))));
    }
   println!("{:?}", right_side_view(root));
}