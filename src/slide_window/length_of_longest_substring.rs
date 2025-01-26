
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;

        let mut left = 0;
        let mut right = 0;

        let mut cache = HashSet::new();
         
        let chars = s.chars().collect::<Vec<_>>();
      
        for (_, v) in chars.iter().enumerate() {
            // delete util only one v
            while cache.contains(v) {
                 cache.remove(&chars[left]);
                 left += 1 ;
            }
            cache.insert(v);
            max_len = max_len.max(right - left + 1);
            right += 1;
        }
        max_len as i32
}