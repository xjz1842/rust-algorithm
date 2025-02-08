use std::i32;


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

pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>)
 -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut cur = &mut dummy;

    while let (Some(node1), Some(node2)) = (&list1, &list2) {
        if node1.val < node2.val {
            // 把 list1 加到新链表中
            cur.next = list1.take(); 
            cur = cur.next.as_mut()?;
            list1 = cur.next.take();
        } else { 
             // 把 list2 加到新链表中
            cur.next = list2.take();
            cur = cur.next.as_mut()?;
            list2 = cur.next.take();
        };
    }
    cur.next = list1.or(list2);

    dummy.next
}

#[test]
fn merge_two_lists_test() {
    let  list1 = Some(Box::new(ListNode::new(1)));
    let  list2 = Some(Box::new(ListNode::new(2)));
    let mut list = merge_two_lists(list1,list2);
    while list.is_some() {
        println!("{}", list.unwrap().val);
        list = list.unwrap().next;
    }

}