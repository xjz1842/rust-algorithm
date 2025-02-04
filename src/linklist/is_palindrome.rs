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

// 876. 链表的中间结点
fn middle_node(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast: &Option<Box<ListNode>> = head;
    let mut slow = head;
    while fast.is_some() && fast.as_ref()?.next.is_some() {
        slow = &slow.as_ref()?.next;
        fast = &fast.as_ref()?.next.as_ref()?.next;
    }
    #[allow(mutable_transmutes)]
    let slow: &mut Option<Box<ListNode>> = unsafe { std::mem::transmute(slow) };
    slow.take()
}   

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            let nxt = node.next;
            node.next = pre;
            pre = Some(node);
            cur = nxt;
        }
        pre
    }

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut head= head;
    let mid = middle_node(&head);
        let mut head2 = reverse_list(mid);
        while head.is_some() {
            if head.as_ref().unwrap().val != head2.as_ref().unwrap().val {
                return false;
            }
            head = head.unwrap().next;
            head2 = head2.unwrap().next;
        }
        true
}