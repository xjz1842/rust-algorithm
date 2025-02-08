
pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut result = vec![];

    let mut candidate = vec![];

    dfs(&s, &mut candidate, 0, &mut result);

    result
}

fn dfs(s: &String, candidate: &mut Vec<String>, i: usize,
     result: &mut Vec<Vec<String>>) {
    if i == s.len() {
        result.push(candidate.clone());
        return;
    }
    for j in i..s.len() {
        //找到从i到j是回文的字符串
        if valid(&s[i..=j]) {
            candidate.push(s[i..=j].to_string());
            dfs(s, candidate, j+1, result);
            candidate.remove(candidate.len()-1);
        }
    }
}

fn valid(s: &str) -> bool {
    let len = s.len();
    s[0..(len+1)/2].chars().collect::<String>() 
    == s[len/2..len].chars().rev().collect::<String>()
}

#[test]
fn partition_test() {
    let s = "aab".to_string();
    print!("partition {:?}", partition(s));
}
