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

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
    return dfs(root, &mut k).unwrap();
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32)-> Option<i32>{
    match root {
        Some(node) => {
            let mut borrow_node = node.borrow_mut();
            let res_l = dfs(borrow_node.left.take(), k);
            if res_l.is_some() {
                return res_l;
            }
            *k = *k - 1;
            if *k == 0 {
              return Some(borrow_node.val);
            }
            let res_2 = dfs(borrow_node.right.take(), k);
            if res_2.is_some() {
               return res_2;
            }
            None
        },
        None => None
    }
}


#[test]
fn kth_smallest_test() {
    let mut root = Some(Rc::new(RefCell::new(
        TreeNode::new(1))));
    let current = &mut root;

    if let Some(node) = current {
        node.borrow_mut().left = Some(Rc::new(RefCell::new(
            TreeNode::new(2))));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(
            TreeNode::new(3))));
    }
    let  k = 3;
   println!("{}",kth_smallest(root, k));
}