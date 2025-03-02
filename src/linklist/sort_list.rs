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

pub fn sort_list(head: Option<Box<ListNode>>) ->
 Option<Box<ListNode>> {
   if head.is_none() || head.as_ref()?.next.is_none() {
       return head;
   }
   let mid= middle_node(&head);
   
   let head1 = sort_list(head);
   let head2= sort_list(mid);

   return  merge_two_sort_list(head1,
   head2);
}


fn middle_node(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast: &Option<Box<ListNode>> = head;
    let mut slow = head;
    while fast.is_some() && fast.as_ref()?.next.is_some() {
        slow = &slow.as_ref()?.next;
        fast = &fast.as_ref()?.next.as_ref()?.next;
    }
    // 把 slow 从 &Option<Box<ListNode>> 强转成 &mut Option<Box<ListNode>>
    #[allow(mutable_transmutes)]
    let slow: &mut Option<Box<ListNode>> = unsafe { std::mem::transmute(slow) };
    slow.take() // 断开中间节点和前一个节点的连接
}

fn merge_two_sort_list(mut head1: Option<Box<ListNode>>,
mut head2: Option<Box<ListNode>>)-> Option<Box<ListNode>> {
  let mut dummy =  ListNode::new(0);

  let mut cur = &mut dummy;

  while let (Some(node1), Some(node2)) = 
  (&head1, &head2) {
        if node1.val <= node2.val {
            cur.next = head1.take();
            cur = cur.next.as_mut()?;
            head1 = cur.next.take();
        } else {
            cur.next = head2.take();
            cur = cur.next.as_mut()?;
            head2 = cur.next.take();
        }
  }
  cur.next = head1.or(head2);
  dummy.next
}



#[test]
fn sort_list_test() {
    let mut node: ListNode = ListNode::new(2);
    node.next = Some(Box::new(ListNode::new(1)));
    let head = Some(Box::new(node));
    println!("{:?}",sort_list(head));
}
