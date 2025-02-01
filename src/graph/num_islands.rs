pub fn num_islands(grids: Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    let mut grid = grids;
    let rows = grid.len();
    let cols = grid[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                dfs(&mut grid, r, c); // 把这个岛填充'*'，这样后面遍历到的 '1' 一定是新的岛
                result += 1;
            }
        }
    }
    result
}
fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    // 出界，或者不是 '1'，就不再往下递归
    if i >= grid.len() || j >= grid[i].len() || grid[i][j] != '1' {
        return;
    }
    grid[i][j] = '*'; // 插旗！避免来回横跳无限递归
    if j >= 1 {
        dfs(grid, i, j - 1); // 往左走
    }
    dfs(grid, i, j + 1); // 往右走
    if i >= 1 {
        dfs(grid, i - 1, j); // 往上走
    }
    dfs(grid, i + 1, j); // 往下走
}
