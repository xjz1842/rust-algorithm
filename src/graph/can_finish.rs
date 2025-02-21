use std::collections::VecDeque;



//拓扑排序 BFS
pub fn can_finish(mut num_courses:i32, prerequisites: Vec<Vec<i32>>) -> bool {
    // 有向图 存储入度
    let mut indegree = vec![0;num_courses as usize];

    //存储图
    let mut graph : Vec<Vec<i32>> =  vec![vec![];num_courses as usize];

    //BFS 需要的使用的队列
    let mut deq = VecDeque::new();
    
    for pre in prerequisites {
        //入度加 1
        indegree[pre[0] as usize] += 1;
        // 1 入度 0 
        graph[pre[1] as usize].push(pre[0]);
    }
    
    for (course,degree) in indegree.iter().enumerate() {
         if *degree == 0 {
             deq.push_back(course as i32);
         }
    }

    //找到入度为0 开始BFS
     while !deq.is_empty() {
         for _ in 0..deq.len() {
            num_courses -= 1;
             let front = deq.pop_front().unwrap();
             for next in &graph[front as usize] {
                indegree[*next as usize] -= 1;
                if indegree[*next as usize] == 0 {
                    deq.push_back(*next as i32);
                 } 
             }
         }
     }
     num_courses == 0   
}


#[test]
fn can_finish_test() {
    let num_courses = 5;
    let prerequisites = vec![vec![1,4],vec![2,4],
    vec![3,1],vec![3,2]];

    assert_eq!(true,can_finish(num_courses, prerequisites));
}
