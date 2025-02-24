use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug,PartialEq, Eq)]
struct Num {
    num: i32,
    count: i32,
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Num {
    fn cmp(&self, other: &Self) -> Ordering {
        // 实现自定义比较逻辑
        match self.count.cmp(&other.count) {
            Ordering::Equal => self.num.cmp(&other.num),
            order => order,
        }
    }
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut  map = HashMap::new();

        for num in nums {
           let v =  map.entry(num).or_insert(1);
           *v += 1;
        }
        let mut heap = BinaryHeap::new();

        for (key,value) in map {
            heap.push(Num{num:key,count:value});
        }
        for _ in 0..k{
            result.push(heap.pop().unwrap().num);
        }
        result
}

#[test]
fn top_k_frequent_test() {
    let nums = vec![1];
    let k = 2;
    println!("{:?}", top_k_frequent(nums, k));

}