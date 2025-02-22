
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    // 大根堆
     let mut heap = vec![];
     let len = nums.len();
     for e in nums {
        if heap.len() < (len - k as usize + 1) {
            heap.push(e);
            sift_up(&mut heap);
        } else {
            if heap[0] >= e {
                heap[0] = e;
                sift_down(&mut heap);
            } else {
               continue;
            }
        }
     }
     heap[0]
}


fn sift_up(heap: &mut Vec<i32>) {
   let mut insert_idx = heap.len() - 1;
   if insert_idx < 1 {
      return;
   }
   let mut parent_idx =  (insert_idx -  1) / 2;

   while 
     heap[parent_idx] < heap[insert_idx] {
       heap.swap(parent_idx, insert_idx);

      insert_idx = parent_idx;
      if insert_idx < 1 {
         break;
      }
      parent_idx = (insert_idx - 1) / 2;
   }
}

fn sift_down(heap: &mut Vec<i32>) {
   let last_idx = heap.len()-1;
   heap.swap(0, last_idx);
 
   let mut parent_idx = 0;
   let mut left_idx = 2 * parent_idx + 1;
   let mut right_idx = 2 * parent_idx + 2;

  while left_idx < heap.len() {
      let max_idx = if right_idx < heap.len() {
           if heap[right_idx] > heap[left_idx] {
                right_idx
           } else {
                left_idx
           }
      } else {
         left_idx
      };
     if heap[max_idx] > heap[parent_idx] {
        heap.swap(max_idx, parent_idx);
     }
     parent_idx = max_idx;
     left_idx = 2 * parent_idx + 1;
     right_idx = 2 * parent_idx + 2;
  }
}


pub fn find_kth_largest_1(nums: Vec<i32>, k: i32) -> i32 {
    // 桶排序
     let mut bucket = vec![0;20001];

     let mut k = k;
     for num in nums.iter() {
        bucket[*num as usize + 10000] += 1;
     }

     for i in (0..bucket.len()).rev(){
        let c = bucket[i];
            if k > c  {
                k -= c;
            } else {
               return i as i32 - 10000;
            }
         }
     -1
}

#[test]
fn find_kth_largest_test() {
    let nums = vec![3,2,1,5,6,4];
    let k = 2;
    assert_eq!(5,find_kth_largest(nums, k));
    let nums = vec![3,2,1,5,6,4];
    assert_eq!(5, find_kth_largest_1(nums, k));
}