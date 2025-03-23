
pub fn candy(ratings: Vec<i32>) -> i32 {
      let mut result = vec![1; ratings.len()];
    
     for i in 1..ratings.len() {
         if  ratings[i-1] < ratings[i]  {
             result[i] = result[i-1] + 1;
         }
     }
     for i in (0..ratings.len()-1).rev() {
         if ratings[i] > ratings[i+1] {
             result[i] = result[i].max(result[i+1] + 1);
         }
     }

     return result.iter().sum();
}

#[test]
fn test() {
    assert_eq!(candy(vec![1,0,2]), 5);
    assert_eq!(candy(vec![1,2,2]), 4);
}