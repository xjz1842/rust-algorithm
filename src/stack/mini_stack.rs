
struct MinStack {
   st : Vec<(i32,i32)>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {st : vec![(0,i32::MAX)]}
    }
    
    fn push(&mut self, val: i32) {
       self.st.push((val,self.get_min().min(val)));
    }
    
    fn pop(&mut self) {
        self.st.pop();
    }
    
    fn top(&self) -> i32 {
        self.st.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.st.last().unwrap().1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

 #[test]
 fn mini_stack_test () {
      let mut obj = MinStack::new();
      obj.push(1);
      obj.pop();
      let ret_3 = obj.top();
      let ret_4: i32 = obj.get_min();
      println!(" {}  {} ",ret_3, ret_4);
 }

