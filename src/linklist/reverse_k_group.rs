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

pub fn reverse_k_group( head: Option<Box<ListNode>>, k: i32) -> 
Option<Box<ListNode>> {
  let mut len = 0;
  let mut head = head;
  let mut tail = &mut head;

  let tail_node= loop {
      match tail {
          Some(node) => {
            len += 1;
            if len == k {
              break node.next.take()
            }
            tail = &mut node.next
          },
          None => break None,
      }
  };
  
  if len == k {
     reverse_list(head, reverse_k_group(tail_node, k))
  } else {
     head
  }
}

fn reverse_list(head: Option<Box<ListNode>>,
  tail: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
   let mut head = head;
   let mut pre: Option<Box<ListNode>> = tail;
   // reverse
   while let Some(mut node) =  head {
       head = node.next.take();
       node.next = pre;
       pre = Some(node);
   }
   pre
}

#[test]
fn reverse_k_group_test() {
    let mut head: ListNode = ListNode::new(1);
    head.next = Some(Box::new(ListNode::new(2)));
    
    if let Some(node) = head.next.as_mut() {
        node.next = Some(Box::new(ListNode::new(3)));
    }
    let k = 2;
    println!("{:?}", reverse_k_group(Some(Box::new(head)),
    k));
}

