pub fn score_of_string(s: String) -> i32 {
        
    let len = s.len();
    let mut result = 0_i32;
    for i in 0..len-1 {
        let cur = s.chars().nth(i).unwrap() as u8;
        let next = s.chars().nth(i+1).unwrap() as u8;
        
         result +=  cur.abs_diff(next) as i32;
    }
    result
}

#[test]
fn score_of_string_test() {
    let s = "AB".to_string();
    let result = score_of_string(s);
    assert_eq!(result, 1);
}


