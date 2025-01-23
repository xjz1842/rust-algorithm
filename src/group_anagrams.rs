use std::collections::HashMap;

/**
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap< String, Vec<String> > = HashMap::new();
        for s in strs {
            let mut cv : Vec<char> = s.chars().collect();
            cv.sort();
            let os = cv.iter().collect::<String>();
            let vec: Vec<String> = Vec::new();
            let val = map.entry(os).or_insert(vec);
            val.push(s);
        }
        let mut res = Vec::new();
        for val in map.values() {
            res.push(val.to_owned());
        }
        res
    }
}
 */

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result : Vec<Vec<String>> = vec![];
        let mut hash_map : HashMap<String, Vec<String>>  = HashMap::new();

         for s in strs {
             let mut chars : Vec<char> = s.chars().collect();
             chars.sort();
             let key = chars.iter().collect::<String>();
             let value: Vec<String> = Vec::new();
             let v = hash_map.entry(key).or_insert(value);
             v.push(s);
         }
         for v in hash_map.values() {
            result.push(v.to_owned());
         }
         result
}