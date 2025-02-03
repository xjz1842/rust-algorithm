
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
     let mut result = vec![];
     let mut candidate = vec![];
     dfs(&nums, 0 ,&mut candidate, &mut result);
     result
}

fn dfs(nums: &Vec<i32>, index : usize, 
    candidate: &mut Vec<i32>,
    result :&mut Vec<Vec<i32>>) {

    result.push(candidate.clone());
    
    if index == nums.len() {
       return;
    }
    for i in index..nums.len() {
     candidate.push(nums[i]);
     dfs(nums, i + 1, candidate, result);
     candidate.pop();
   }
}