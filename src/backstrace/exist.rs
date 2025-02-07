


pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
   let rows = board.len();
   let cols = board[0].len();
    
   let mut visited: Vec<Vec<bool>> = vec![];

      
   let start = word.chars().next().unwrap();

   for _ in 0..rows {
      visited.push(vec![false;cols]);
   }
    // down right up left
    let direction =  vec![(1,0),(0,1),(-1,0),(0,-1)];     

    for r in 0..rows {
        for c in 0..cols {
            if board[r][c] == start {
                if word.len() == 1 {
                    return true;
                }
                visited[r][c] = true;
                let mut candidate = start.to_string();
                if dfs(&board,&mut visited,&direction,r as i32,c as i32,&mut candidate, &word) {
                    return true;
                }
                visited[r][c] = false
            }
        }
    }
    false
}

fn dfs(board: &Vec<Vec<char>>,visited: &mut Vec<Vec<bool>>,
    directions: &Vec<(i32,i32)>,r : i32,
     c : i32, candidate: &mut String, word: &String)-> bool {
     
   for direction in directions.iter() {
        let new_r = r as i32 + direction.0 ;
        let new_c = c as i32 + direction.1;
        if new_r >= 0 && new_r < board.len() as i32 &&
           new_c >= 0 && new_c < board[0].len() as i32 
            && !visited[new_r as usize][new_c as usize] {

            if candidate.len() < word.len() && 
               board[new_r as usize][new_c as usize] 
               == word.chars().nth(candidate.len()).unwrap(){
               candidate.push(board[new_r as usize][new_c as usize]);

               if word == candidate {
                  println!(" {} ",candidate);
                  return true;
               }
               visited[new_r as usize][new_c as usize] = true;
               if dfs(board,visited, directions, new_r, new_c, candidate, word) {
                  return true;
               }
               candidate.remove(candidate.len()-1);
               visited[new_r as usize][new_c as usize] = false;
            } 
         }
   }
    false
}

#[test]
fn exist_test(){
  let board = vec![vec!['A']];
  let word = "A".to_string();
  assert_eq!(true,exist(board, word));
}