use std::collections::VecDeque;


pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = vec![];
  
    let mut deque :VecDeque<usize> = VecDeque::new();
    
    for (i, v) in nums.iter().enumerate() {
        // 单调递减 index
        while !deque.is_empty() && nums[deque[deque.len()-1]] <= *v {
            deque.pop_back();
        }
        deque.push_back(i);

        if (i - deque[0]) as i32 >= k {
            deque.pop_front();
        } 
        if i as i32 >= (k - 1) {
            result.push(nums[deque[0]]);
        }
    }
    result
}