
#[warn(dead_code)]

use std::collections::HashMap;

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//      let mut result = Vec::new();
//      let len: usize = nums.len();
//      for i in 0..len {
//         for j in i+1..len {
//             if (nums[i] + nums[j]) == target {
//                 result.push(i as i32);
//                 result.push(j as i32);
//             } 
//         }
//      }
//      result
// }

pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let len: usize = nums.len();

    let mut map : HashMap<i32,i32> = HashMap::new();

    for i in 0..len {
        map.insert(nums[i], i as i32, );
    }
    for j in 0..len {
      if let Some(i) = map.get(&(target - nums[j])){
        if  *i != j as i32 {
            result.push(j as i32);
            result.push(*i);
            return result;
        }
      }
    }
    result
}