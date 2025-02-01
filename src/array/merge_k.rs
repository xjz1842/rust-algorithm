

// 合并两个有序向量
fn merge_two_sorted_vecs(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::with_capacity(vec1.len() + vec2.len());
    let mut i = 0;
    let mut j = 0;
 
    while i < vec1.len() && j < vec2.len() {
        if vec1[i] <= vec2[j] {
            merged.push(vec1[i]);
            i += 1;
        } else {
            merged.push(vec2[j]);
            j += 1;
        }
    }
    // 将剩余元素添加到合并后的向量中
    merged.extend_from_slice(&vec1[i..]);
    merged.extend_from_slice(&vec2[j..]);
    merged
}


pub fn merge_k_sorted_vecs(mut vecs: Vec<Vec<i32>>) -> Vec<i32> {
    if vecs.len() == 1 {
        // 只剩一个向量时，直接返回
        return vecs.pop().unwrap();
    }
    let mid = vecs.len() / 2;
    let left_half: Vec<Vec<i32>> = vecs.drain(..mid).collect();
    let right_half: Vec<Vec<i32>> = vecs.into_iter().collect();
 
    // 递归地合并左右两半
    let left_merged = merge_k_sorted_vecs(left_half);
    let right_merged = merge_k_sorted_vecs(right_half);
 
    // 合并最终的两个结果向量
    merge_two_sorted_vecs(&left_merged, &right_merged)
 }