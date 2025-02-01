

pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
      let mut result : Vec<Vec<i32>> = Vec::new();
      dfs(& mut nums, 0, & mut result);
      result
}

fn dfs(nums :&mut Vec<i32>, index: usize, result : & mut Vec<Vec<i32>>) {
    if index == nums.len() { 
       result.push(nums.to_vec());
       return;
    }
    for i in index.. nums.len() {
        nums.swap(i,index);
        dfs(nums, index+1, result);
        nums.swap(i, index);
    }
}
