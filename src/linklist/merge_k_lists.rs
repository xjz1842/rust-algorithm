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

use std::{cmp::Ordering, collections::BinaryHeap};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap: BinaryHeap<Box<ListNode>> = BinaryHeap::new();

    let len = lists.len();
    if len == 0 {
        return None;
    }
    let mut dummy = ListNode::new(0);
    let mut cur = &mut dummy;
    for node in lists{
        if let  Some(v) =  node {
            heap.push(v);
        }
    }

   while !heap.is_empty() {
      if let Some(mut  v) = heap.pop() {
        if let Some(next) =  v.next.take() {
            heap.push(next);
        }
        cur.next = Some(v);
        cur = cur.next.as_mut()?;
      }
   }
   dummy.next
}

#[test]
fn merge_k_lists_test(){
    let  list1 = Some(Box::new(ListNode::new(1)));
    let  list2 = Some(Box::new(ListNode::new(2)));
    let mut list = merge_k_lists(vec![list1,list2]);
    while let Some(node) = &list {
        println!("{}", node.val);
        list = list.take().unwrap().next;
    }
} 