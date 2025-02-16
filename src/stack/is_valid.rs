

pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        if c == ')' {
            if stack.is_empty()
             || stack.pop().unwrap() != '(' {
               return false; 
            }
        } else if c == ']' {
            if stack.is_empty()
            || stack.pop().unwrap() != '[' {
                return false;
            }
        }else if c == '}' {
            if stack.is_empty()
            || stack.pop().unwrap() != '{' {
                return false;
            }
        } else {
            stack.push(c);
        }
    }  
    stack.is_empty()
}

#[test]
fn is_valid_test(){
   let s = "([])".to_string();
   assert_eq!(true, is_valid(s));

}