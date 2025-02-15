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

pub fn add_two_numbers(l1: Option<Box<ListNode>>, 
    l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut cur = &mut dummy;

    let mut carry = 0;
    let mut l1 = l1;
    let mut l2 =  l2;

    while let (Some(node1), Some(node2)) = (& l1,& l2) {
         let mut sum = node1.val +  node2.val + carry;
         carry = 0;
         if sum >= 10 {
            carry = 1;
            sum -= 10;
         } 
         l1 = node1.next.clone();
         l2 = node2.next.clone();
         match cur {
             Some(node) => {
                  node.next = Some(Box::new(ListNode::new(sum)));
                  cur = &mut node.next;
             },
             None => (),
         } 
    }

    while let Some(node1) = & l1 {
        let mut sum: i32 = node1.val + carry;
         carry = 0;
         if sum >= 10 {
            carry = 1;
            sum -= 10;
         } 
         l1 = node1.next.clone();
         match cur {
            Some(node) => {
                 node.next = Some(Box::new(ListNode::new(sum)));
                 cur = &mut node.next;
            },
            None => (),
        } 
    }

    while let Some(node2) = &mut l2 {
        let mut sum = node2.val + carry;
         carry = 0;
         if sum >= 10 {
            carry = 1;
            sum -= 10;
         } 
         l2 = node2.next.clone();
         match cur {
            Some(node) => {
                 node.next = Some(Box::new(ListNode::new(sum)));
                 cur = &mut node.next;
            },
            None => (),
        } 
    }
    if carry == 1 {
        match cur {
            Some(node) => {
                 node.next = Some(Box::new(ListNode::new(carry)));
                 cur = &mut node.next;
            },
            None => (),
        } 
    }
    dummy.unwrap().next

}


#[test]
fn add_two_numbers_test() {
    let  list1 = Some(Box::new(ListNode::new(1)));
    let  list2 = Some(Box::new(ListNode::new(2)));
    let mut list = add_two_numbers(list1,list2);
    while let Some(node) = &list {
        println!("{}", node.val);
        list = list.take().unwrap().next;
    }
}
