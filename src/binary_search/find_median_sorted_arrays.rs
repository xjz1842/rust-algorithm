

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (nums1,nums2) =  if nums1.len() > nums2.len() {
        (nums2,nums1)
    } else {
        (nums1,nums2)
    };
    
    let len1 = nums1.len();
    let len2 = nums2.len();
    // 分割线昨天的元素
    let total_left = (len1 + len2 + 1) / 2;

    // 在[0,m] 里查找恰当的分割线
    // 使得 nums1[i-1] <= nums[j]   && nums[j-1] <= nums[i]
    let mut left = 0;
    let mut right = len1;

    while left <= right {
        let i: usize = left + (right - left) / 2;
        let j = total_left - i;
        if j != 0 && i != len1 && nums2[j - 1] > nums1[i] {
            left = i + 1;
        } else if i != 0 && j != len2 && nums1[i-1] > nums2[j] {
            right = i;
        } else{
            // 边界
            let left_max ;
            if i == 0 {
                left_max = nums2[j-1];
            } else if j == 0 {
               left_max = nums1[i-1];
            }else {
                left_max = nums1[i-1].max(nums2[j-1]);
            }
            if (len1 + len2) & 1 == 1{
                return left_max as f64;
            }
            let  right_min;
            if i == len1 {
                right_min = nums2[j];
            } else if j == len2 {
                right_min = nums1[i];
            } else {
                right_min = nums1[i].min(nums2[j]);
            }
           return (left_max + right_min) as f64 / 2.0;
        }
    }
   0.0
}


#[test]
fn find_median_sorted_arrays_test() {
    let nums1 = vec![1,3];
    let nums2 = vec![2];
    assert_eq!(2.0,find_median_sorted_arrays(nums1,nums2));
}