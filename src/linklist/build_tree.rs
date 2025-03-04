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

pub fn build_tree(preorder: Vec<i32>, 
    inorder: Vec<i32>) ->
     Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() { // 空节点
       return None;
   }
  let left_size = inorder.iter()
  .position(|x|*x == preorder[0]).unwrap();

  let pre_left = preorder[1..1+left_size].to_vec();
  let pre_right = preorder[left_size+1..].to_vec();
  
  let in_left = inorder[..left_size].to_vec();
  let in_right = inorder[left_size+1..].to_vec();
  
  let left = build_tree(pre_left, in_left);
  let right = build_tree(pre_right, in_right);
        
 return Some(Rc::new(RefCell::new(TreeNode{val:preorder[0],left,right})));
}

#[test]
fn build_tree_test(){
   let preorder = vec![3,9,20,15,7];
   let inorder = vec![9,3,15,20,7];

   println!("{:?}", build_tree(preorder, inorder));
}