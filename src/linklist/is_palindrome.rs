// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut copy_head: Option<Box<ListNode>> = head;
    let mut list = vec![];
    while let Some(node) = copy_head {
        list.push(node.val);
        copy_head = node.next;
    }
     let len = list.len();
     for i in 0..len/2 {
        if list[i] != list[len - i - 1] {
            return false;
        }
     }
     true
}