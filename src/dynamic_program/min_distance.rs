

pub fn min_distance(word1: String, word2: String) -> i32 {
    let (word_1, word_2) = (word1.as_bytes(), word2.as_bytes());
    let len1= word_1.len();
    let len2= word_2.len();
    
    let mut dp = vec![vec![0;len2+1];len1+1];

    for j in 0..=len2 {
        dp[0][j] = j;
    }
    
    for i in 0..=len1 {
        dp[i][0] = i;
    }
   
    for i in 1..=len1 {
        for j in 1..=len2 {
            if word_1[i-1] == word_2[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = (dp[i][j-1]).min(dp[i-1][j]).min(dp[i-1][j-1]) + 1;
            }
        }
    }
   dp[len1][len2] as i32
}


#[test]
fn min_distance_test() {
    let word1 = "intention".to_string();
    let word2 = "execution".to_string();
    assert_eq!(5,min_distance(word1,word2));
}