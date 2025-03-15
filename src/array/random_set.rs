use std::collections::HashMap;

use rand::Rng;

struct RandomizedSet {
  list : Vec<i32>,
  map : HashMap<i32, i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self
         {
          list : Vec::new(),
          map : HashMap::new()
         }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }else {
            self.list.push(val);
            self.map.insert(val, self.list.len() as i32 -1);
            return true;
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            return false;
        }
        let del_index: &i32 = self.map.get(&val).unwrap();
        let del_index = *del_index;
        if del_index== (self.list.len() as i32 - 1) {
            self.list.pop();
            self.map.remove(&val);
            return true;
        } else {
            let last_index = self.list.len() - 1 ;
            let last_val = self.list[last_index];
            self.map.insert(last_val, del_index);    
            self.list[del_index as usize] = last_val;
            self.list.pop();    
            self.map.remove(&val);
            return true;
        }
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0,self.list.len());
        return self.list[idx];
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

#[test]
fn random_set_test() {
    let mut obj = RandomizedSet::new();
     let val = 3;
     let ret_1: bool = obj.insert(val);
     let ret_2: bool = obj.remove(val);
    println!("{} {} ", ret_1, ret_2);
}
