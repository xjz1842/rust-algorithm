

pub fn decode_string(s: String) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            if stack.is_empty() {
                stack.push(c);
            } else if c == ']' {
               let mut temp = "".to_string();
          
               while let Some(l) =  stack.pop()  {
                  if l == '[' {
                     break;
                  }
                  temp.insert(0,l);
               }
               let mut  repeat_count = "".to_string();
               while let Some(repeat) = stack.last()  {
                    if repeat.is_digit(10) {
                        repeat_count.insert(0, stack.pop().unwrap());
                    }else {
                        break;
                    }
               }
               temp = temp.repeat(repeat_count.parse::<i32>().unwrap() as usize);
        
               for i in temp.chars() {
                  stack.push(i);
                }
            } else {
                stack.push(c);
            }
        }    
        stack.iter().collect()

}

#[test]
fn decode_string_test() {
    let s = "2[abc]3[cd]ef".to_string();

    assert_eq!("abcabccdcdcdef",decode_string(s));
}