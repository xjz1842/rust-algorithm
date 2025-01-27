

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
   let mut result : Vec<Vec<i32>> = Vec::new();
   
   let mut intervals = intervals;
   // sort
   intervals.sort_by(|a,b| (a[0])
   .cmp(&b[0]));
   
   for interal in intervals {
        let last =  result.last_mut();
        if let Some(v) = last {
            if v[1] < interal[0] {
               result.push(interal);
            } else {
               v[1] = interal[1].max(v[1]);
            }
        } else {
           // None 
           result.push(interal);
        }
   }
   result
}