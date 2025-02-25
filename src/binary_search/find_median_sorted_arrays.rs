
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (nums1, nums2) = if nums1.len() > nums2.len() {
        (nums2, nums1)
    } else {
        (nums1, nums2)
    };

    let len1 = nums1.len();
    let len2 = nums2.len();

    let mut left : i32 = -1;
    let mut right : i32 = len1 as i32;

    while left + 1 < right {
        let i: usize =  (right + left)as usize / 2;
        let j = (len1 + len2 - 1) / 2  - i;
        if nums2[j] >= nums1[i] {
            left = i as i32;
        } else  {
            right = i as i32;
        }
    }
    let i = left;
    let j = (len1 + len2 + 1) as i32 / 2 - 2 - i;
    // 此时 left 等于 right-1
    // a[left] <= b[j+1] 且 a[right] > b[j]，所以答案是 i=left
    let left_1 = if i >= 0 {
          nums1[i as usize]
    }  else {
         i32::MIN
    };
    let left_2 = if j >= 0 {
        nums2[j as usize]
   }  else {
       i32::MIN
   };
   let left_max = left_1.max(left_2);

   let right_1 = if i + 1 < len1 as i32{
        nums1[i as usize +1]
   } else {
      i32::MAX
   };

   let right_2 = if j + 1 < len2 as i32 {
        nums2[j as usize +1]
   } else {
      i32::MAX
   };
   let right_min = right_1.min(right_2);

   if (len1 + len2) & 1 == 1 {
      return left_max as f64;
   } else {
      return (left_max + right_min) as f64 / 2.0;
   }
}

#[test]
fn find_median_sorted_arrays_test() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(2.0, find_median_sorted_arrays(nums1, nums2));
}
