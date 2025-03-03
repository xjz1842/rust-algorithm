
pub fn longest_palindrome(s: String) -> String {
    let chars : Vec<char> = s.chars().collect();
    let len = chars.len();
    if len <= 1 {
        return s;
    }
    //表示字符串从 i 到 j 这段是否为回文
    let mut dp = vec![vec![false;len];len];
    let mut max_start = 0;  //最长回文串的起点
    let mut max_end = 0;    //最长回文串的终点
    let mut max_len = 1;  //最长回文串的长度

    for j in 1..len {
        for i in 0..j {
            if chars[i] == chars[j] &&
              ( (j - i) <= 2  ||  dp[i+1][j-1] ) {
                dp[i][j] = true;
                if (j - i + 1) > max_len {
                    max_len = j - i + 1;
                    max_start = i;
                    max_end = j;
                }
            }
        }
    }
    chars[max_start..=max_end].iter().collect()
}

#[test]
fn longest_palindrome_test() {
  let str = "aaaa".into();
  println!("{}", longest_palindrome(str));
}