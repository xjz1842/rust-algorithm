


pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
     let row = grid.len();
     let col = grid[0].len();  
     let mut no_fresh_oranges = vec![];
     let mut fresh_oranges = vec![];
     let mut grid = grid;
     let mut  min_minutes = 0;

     get_stat_oranges(&grid, &mut no_fresh_oranges, &mut fresh_oranges);
     let mut last_fresh_count = fresh_oranges.len();
     while !fresh_oranges.is_empty() {
         min_minutes += 1;
         // 经过轮腐烂
         for r in no_fresh_oranges.iter(){
              if r.0  >= 1 && grid[r.0 - 1][r.1] == 1 {
                grid[r.0 - 1][r.1]  = 2;
              }
              if (r.0 + 1) < row && grid[r.0 + 1][r.1] == 1{
                grid[r.0 + 1][r.1] = 2;
             }
             if r.1 >= 1  &&  grid[r.0][r.1 - 1] == 1{
                grid[r.0][r.1 - 1] = 2;
             }
             if r.1 + 1 < col  &&  grid[r.0][r.1 + 1] == 1{
                grid[r.0][r.1 + 1] = 2;
             }
         }
         no_fresh_oranges.clear();
         fresh_oranges.clear();
         get_stat_oranges(&grid, &mut no_fresh_oranges, &mut fresh_oranges);
         println!("{:?} {:?}", last_fresh_count, fresh_oranges);
         if last_fresh_count == fresh_oranges.len()
           && last_fresh_count > 0 {
            return -1;
         }
         last_fresh_count = fresh_oranges.len();
     }
     min_minutes
}

fn get_stat_oranges(grid: &Vec<Vec<i32>>,
    no_fresh_oranges :&mut Vec<(usize,usize)>,
    fresh_oranges : &mut Vec<(usize,usize)>) {
    for r in 0..grid.len(){
        for c in 0..grid[0].len() {
            if grid[r][c] == 2  {
                no_fresh_oranges.push((r,c));
            }
            if grid[r][c] == 1  {
                fresh_oranges.push((r,c));
            }
         }
     }   
}


#[test]
fn oranges_rotting_test() {
    let grid = vec![vec![2,1,1],vec![0,1,1],vec![1,0,1]];

    assert_eq!(-1,oranges_rotting(grid));
}