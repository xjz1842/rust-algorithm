pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut citations = citations;
    let len = citations.len();

    citations.sort();
    for i in (0..len).rev() {
            // 当论文的数量 小于 引用的数量时停止
        if citations[i] < (len - i) as i32 {
            return (len - i) as i32 -1;
        }
    }
    len as i32
}


#[test]
fn h_index_test() {
   let citations  = vec![3,0,6,1,5];
   assert_eq!(3,h_index(citations));
}

