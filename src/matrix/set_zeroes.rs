use std::collections::HashSet;

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
     let mut row: HashSet<i32>= HashSet::new();
     let mut col: HashSet<i32>= HashSet::new();
     let r_len = matrix.len();
     let c_len = matrix[0].len();
     for i in 0..r_len {
       for  j in 0..c_len {
            if matrix[i][j] == 0 {
                row.insert(i as i32);
                col.insert(j as i32);
            }        
       }
     }
     for i in 0..r_len {
        for  j in 0..c_len {
             if row.contains(&(i as i32)) || col.contains(&(j as i32)) {
                 matrix[i][j] = 0;
             }        
        }
      }
}