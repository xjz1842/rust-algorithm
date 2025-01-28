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

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let l_len = dfs(&node.left, ans) + 1; // 左子树最大链长+1
            let r_len = dfs(&node.right, ans) + 1; // 右子树最大链长+1
            *ans = (*ans).max(l_len + r_len); // 两条链拼成路径
            return l_len.max(r_len); // 当前子树最大链长
        }
        -1
    }
    let mut ans = 0;
    dfs(&root, &mut ans);
    ans
}

