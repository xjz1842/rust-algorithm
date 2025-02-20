pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let len1 = text1.len();
    let len2 = text2.len();

    let text_1: Vec<char> = text1.chars().collect();
    let text_2: Vec<char> = text2.chars().collect();

    // i,j 是代表text1在(0,i)  text2 在(0,j) 位置上 最大的为
    let mut dp = vec![vec![0; len2]; len1];

    for r in 0..len1 {
        for c in 0..len2 {
            if text_1[r] == text_2[c] {
                if r >= 1 && c >= 1 {
                    dp[r][c] = dp[r][c].max(dp[r - 1][c - 1] + 1);
                } else {
                    dp[r][c] = 1;
                }
            } else {
                if r >= 1 {
                    dp[r][c] = dp[r][c].max(dp[r - 1][c]);
                }
                if c >= 1 {
                    dp[r][c] = dp[r][c].max(dp[r][c - 1]);
                } 
            }
        }
    }
    dp[len1 - 1][len2 - 1]
}

#[test]
fn longest_common_subsequence_test() {
    let text1 = "psnw".to_string();
    let text2 = "vozsh".to_string();

    assert_eq!(3, longest_common_subsequence(text1, text2));
}
