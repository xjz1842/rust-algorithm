use std::time;


pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();

    dfs(
        0,
        n as usize,
        &mut vec![vec!['.'; n as usize]; n as usize],
        &mut result,
    );

    result
}

fn dfs(row: usize, n: usize, candidate: &mut Vec<Vec<char>>, result: &mut Vec<Vec<String>>) {
    if row == n {
          result.push(
                candidate
                    .clone()
                    .iter()
                    .map(|x| x.iter().map(|y| y.to_owned()).collect::<String>())
                    .collect::<Vec<String>>(),
            );
        return;
    }

    for i in 0..n {
        if valid(candidate, row as i32 ,i as i32) {
            candidate[row][i] = 'Q';
            dfs(row + 1, n, candidate, result);
            candidate[row][i] = '.';
        }
    }
}

fn valid(candidate: &mut Vec<Vec<char>>,i: i32, j : i32) -> bool {
    let rows = candidate.len();
    let cols = candidate[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if candidate[row][col] == 'Q' {
                if row as i32 == i  || j == col as i32 || i32::abs(i - row as i32) 
                == i32::abs(j - col as i32) {
                    return false;
                }
            }
        }
    }
    true
}

#[test]
fn solve_n_queens_test() {
    let n = 9;
    let start = time::Instant::now();

    println!(" {:?}", solve_n_queens(n));

    print!("{:?}", time::Instant::now() - start);
}
