// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
 

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  head.and_then(|mut n| {
    match n.next {
        Some(mut m) => {
            n.next = swap_pairs(m.next);
            m.next = Some(n);
            Some(m)
        },
        None => Some(n)
    }
})
}

#[test]
fn swap_pairs_test(){
    let mut node: ListNode = ListNode::new(1);
    node.next = Some(Box::new(ListNode::new(2)));
    let head = Some(Box::new(node));
    println!("{:?}", swap_pairs(head));
}