
   
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

      let mut last_idx = nums1.len() - 1;

      let mut i = m - 1;
      let mut j = n - 1;

      while i >= 0 && j >= 0 {
          if nums1[i as usize] > nums2[j as usize] {
               nums1[last_idx] = nums1[i as usize];
               i -= 1;
          } else {
            nums1[last_idx] = nums2[j as usize  ];
            j -= 1;
          }
          last_idx -= 1;
      }

      while j >= 0 {
          nums1[last_idx] = nums2[j as usize];
          j -= 1;
          if last_idx == 0{
            break;
          }
          last_idx -= 1;
      }
}


#[test]
fn merge_test(){
    let mut nums1 = vec![0];
    let mut nums2  = vec![1];
    merge(&mut nums1,0,&mut nums2,1);
    println!("{:?}", nums1);
}