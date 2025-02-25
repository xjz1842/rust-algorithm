pub fn longest_valid_parentheses(s: String) -> i32 {
    // 0 到 i的字符串，能到得到最大字符串
    let mut _dp = vec![0; s.len()];
    let chars: Vec<char> = s.chars().collect();

    for i in 1..chars.len() {
        if chars[i] == ')' {
            if chars[i - 1] == '(' {
               _dp[i] = if i >= 2 { _dp[i - 2] + 2 } else { 2 };
            } else {
                if i >= _dp[i- 1] + 1 {
                    let pre_idx = i - _dp[i - 1] - 1;
                    if chars[pre_idx] == '(' {
                        _dp[i] = if pre_idx >= 1 {
                            _dp[i - 1] + _dp[pre_idx - 1] + 2
                        } else {
                            _dp[i - 1] + 2
                        };
                    }
                }
            }
        }
    }
    match _dp.iter().max() {
        Some(v) => *v as i32,
        None => 0,
    }
}

#[test]
fn longest_valid_parentheses_test() {
    let s = "())()())))".to_string();
    assert_eq!(4, longest_valid_parentheses(s));
}
