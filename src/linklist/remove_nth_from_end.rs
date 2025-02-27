
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


pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let dummy = ListNode { val: 0, next: head };
    let mut left = &dummy;
    let mut right = &dummy;

    // stride n step
    for _ in 0..n {
        right = right.next.as_ref()?;
    }

       // 左右指针一起走
       while let Some(ref node) = right.next {
        left = left.next.as_ref()?;
        right = node;
    }
       // 删除倒数第 n 个节点
      // 这里需要把 left 从 &ListNode 强转成 &mut ListNode
      #[allow(mutable_transmutes)]
      let left: &mut ListNode = unsafe { std::mem::transmute(left) };
      left.next = left.next.take()?.next;

      dummy.next
}


#[test]
fn remove_nth_from_end_test() {
    let mut node: ListNode = ListNode::new(1);
    node.next = Some(Box::new(ListNode::new(2)));
    let head = Some(Box::new(node));
    println!("{:?}", remove_nth_from_end(head, 2));
}