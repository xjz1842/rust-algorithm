use std::collections::HashMap;


pub fn letter_combinations(digits: String) -> Vec<String> {
     let mut result = vec![];
  
     if digits.len() == 0 {
        return result;
     }
     let mut candidate = "".to_string();

     let mut map = HashMap::new();
     map.insert('2', "abc".to_string());
     map.insert('3', "def".to_string());
     map.insert('4', "ghi".to_string());
     map.insert('5', "jkl".to_string());
     map.insert('6', "mno".to_string());
     map.insert('7', "pqrs".to_string());
     map.insert('8', "tuv".to_string());
     map.insert('9', "wxyz".to_string());

     let len =  digits.chars().count();

     dfs(&digits.as_bytes(),0, len, &mut candidate, &map, &mut result);
     result
}

fn dfs(digits: &[u8], n: usize,len : usize, candidate: &mut String, digits_map :&HashMap<char,String>,
     result : &mut Vec<String>) {

    if n >= len {
       result.push(candidate.clone()); 
       return;
    }

    let _digit =  digits[n] as char;
    let v =  digits_map.get(&_digit).unwrap();
    
    for (_, s) in v.chars().enumerate() {
        candidate.push(s);

        dfs(digits, n + 1 , len, candidate, digits_map, result);

        candidate.pop();
    }


}
