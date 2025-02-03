
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = vec![];
    let mut candidate = "".to_string();
     dfs(0,0, n, 
        &mut candidate, &mut result);
     result
}

fn dfs(left :i32, right:i32, n : i32,candidate :&mut String, result : &mut Vec<String>) {
    if left > n || right > n {
        return;
    }
    if left == n
      && right == n {
        result.push(candidate.clone());
   }
    if left < n {
        candidate.push('(');
        dfs(left + 1, right, n, candidate, result);
        candidate.pop();
    }
    
    if right < n && left > right{
        candidate.push(')');
        dfs(left, right + 1, n, candidate, result);
        candidate.pop();
    }
}