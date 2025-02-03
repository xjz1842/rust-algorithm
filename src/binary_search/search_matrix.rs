pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let cols = matrix.iter().map(|x|x[0]).collect::<Vec<i32>>();
    let left = binary_search(&cols, target);
    let row = &matrix[left];
    
    let i =  binary_search(row,  target);
    
    return row[i] == target;
}

fn binary_search(vec : &Vec<i32>, target: i32) -> usize {
      let mut left = 0;
      let mut right = vec.len();

      let mut result = 0;

      while left < right {
         let mid = left +  (right - left ) /2;
          if vec[mid] <= target {
            result = mid;
            left = mid + 1;
         } else {
            right = mid;    
         }
      } 
      result 
}