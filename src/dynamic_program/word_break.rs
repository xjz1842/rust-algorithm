

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let len = s.len();
    // 0..i 能由 word_dict 组成单词 
    let mut dp = vec![false;len + 1];
    dp[0] = true;
    for i in 1..=len {
        for word in word_dict.iter() {
            if i == word.len() && s.starts_with(word){
                dp[i] = true;
            } else if i > word.len() && dp[i - word.len()]
                && s[i-word.len()..i] == *word {
                 dp[i] = true;
            }
        }
    }
    dp[len]    
}

#[test]
fn word_break_test() {
  let s: String = "leetcode".to_string();
  let word_dict = vec!["leet".to_string(),
  "code".to_string()];

  assert_eq!(true, word_break(s,word_dict));
}
