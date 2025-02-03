pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut candidate = vec![];
        dfs( &candidates,target,&mut candidate, &mut result);
        result
}

fn dfs(candidates: &Vec<i32>, target: i32, candidate :&mut Vec<i32>,
    result : &mut  Vec<Vec<i32>>) {
    let sum =  candidate.iter().sum::<i32>();

    if sum > target {
        return;
    }
    if sum == target {
        let mut tmp = candidate.to_vec();
        tmp.sort();
        if !result.contains(&tmp) {
            result.push(tmp);
        }
        return;
    }
    for i in candidates {
        candidate.push(*i);
        dfs(candidates, target, candidate, result);   
        candidate.pop();
    }
}