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

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
       let mut result = vec![];
       
       if root.is_none() {
         return result;
       }
       let mut current_level = VecDeque::new();
       current_level.push_back(root.unwrap());
       let mut next_level = VecDeque::new();
      
        loop {
            result.push(current_level.iter().map(|x|x.borrow().val).collect());
            while !current_level.is_empty() {
                let front =  current_level.pop_front().unwrap();
                let mut borrow_front =  front.borrow_mut();
            
                match (borrow_front.left.take(), borrow_front.right.take()) {
                    (Some(left),
                    Some(right) )
                    => {
                        next_level.push_back(left);
                        next_level.push_back(right);
                    },
                    (Some(left),
                     None )=> {
                        next_level.push_back(left);
                    },
                    (None,
                    Some(right) ) => {
                        next_level.push_back(right);
                    }
                    (None,None) => () ,
                }
            }
           if next_level.is_empty(){
               break;
           }
           current_level = next_level.clone();
           next_level.clear();
       }
       result
}

#[test]
fn level_order_test() {
    let mut root = Some(Rc::new(RefCell::new(
        TreeNode::new(1))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(3))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
    }
    println!("{:?}", level_order(root));
}