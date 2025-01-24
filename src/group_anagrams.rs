use std::collections::HashMap;


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