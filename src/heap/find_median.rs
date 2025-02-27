use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    q1 : BinaryHeap<i32>,
    q2 : BinaryHeap<Reverse<i32>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        MedianFinder {
            // 大根堆
            q1: BinaryHeap::new(),
            // 小根堆
            q2: BinaryHeap::new()
        }
    }
    
    fn add_num(&mut self, num: i32) {
        if self.q1.is_empty() {
            self.q1.push(num);
        } else {
           if let Some(top1) = self.q1.peek()  {
                 if num > *top1 {
                      self.q2.push(Reverse(num));
                     while self.q2.len() > self.q1.len() {
                         self.q1.push(self.q2.pop().unwrap().0);
                     }
                 } else {
                    self.q1.push(num);
                    while self.q2.len() + 1 < self.q1.len()  {
                        self.q2.push(Reverse(self.q1.pop().unwrap()));
                    }
                 }
           }
        }
    }
    
    fn find_median(&self) -> f64 {
       if (self.q1.len() + self.q2.len())& 1 == 1 {
           match self.q1.peek() {
               Some(v) => *v as f64,
               None => 0.0
           } 
       } else {
           match (self.q1.peek(), self.q2.peek()) {
               (Some(v),Some(v2)) => {
                     return (*v + v2.0)as f64 / 2.0 ;
               },
               (Some(v),None) => {
                    return *v as f64;
               },
               (None,Some(v2)) => {
                   return v2.0 as f64;
               },
               (None,None) => {
                    0.0
               },
           }
       }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
#[test]
fn find_median_test() {
    let num1 = 3;
    let num2 = 2;
    let mut obj = MedianFinder::new();
    obj.add_num(num1);
    obj.add_num(num2);
    let ret_2: f64 = obj.find_median();
    println!("{}",ret_2 );
}