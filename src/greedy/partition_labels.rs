use std::collections::HashMap;

pub fn partition_labels(s: String) -> Vec<i32> {

   let mut right_max: HashMap<char, usize> = HashMap::new();

   for (idx,c) in s.char_indices() {
      right_max.insert(c,idx); 
   }
   let mut end = 0;
   let mut partitions = vec![];
   let mut last_idx : i32 = -1;
   for (idx,c) in s.char_indices()  {
       if let Some(right_most) = right_max.get(&c) {
           end = end.max(*right_most);
            if idx == end {
               partitions.push(end as i32 - last_idx);
               last_idx = idx as i32;
            }
       } 
   }
   partitions
}


#[test]
fn partition_labels_test(){
    let s =  "ababcbacadefegdehijhklij".to_string();
    println!("{:?}", partition_labels(s));
}