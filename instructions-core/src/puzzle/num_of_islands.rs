use futures::stream::iter;
use std::collections::LinkedList;

const directions: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                count += 1;
                dfs_grid(&mut grid, i, j)
            }
        }
    }
    count
}

fn dfs_grid(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    if grid[i][j] == '0' {
        return;
    }
    grid[i][j] = '0';
    if i >= 1 {
        dfs_grid(grid, i - 1, j);
    }
    if j >= 1 {
        dfs_grid(grid, i, j - 1);
    }
    if i + 1 < grid.len() {
        dfs_grid(grid, i + 1, j);
    }
    if j + 1 < grid[0].len() {
        dfs_grid(grid, i, j + 1);
    }
}

pub fn num_islands_dp(grid: Vec<Vec<char>>) -> i32 {
    use std::iter;
    let mut visited: Vec<Vec<bool>> = Vec::with_capacity(grid.len());
    for _ in 0..grid.len() {
        visited.push(iter::repeat(false).take(grid[0].len()).collect());
    }
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' && !visited[i][j] {
                bfs_search(&grid, &mut visited, i, j);
                count += 1;
            }
        }
    }

    count
}

fn bfs_search(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    use std::collections::LinkedList;
    let mut q = LinkedList::new();
    q.push_back((i, j));
    while !q.is_empty() {
        match q.pop_front() {
            Some((i, j)) => {
                visited[i][j] = true;
                if i + 1 < grid.len() && !visited[i + 1][j] && grid[i + 1][j] == '1' {
                    q.push_back((i + 1, j));
                }
                if i > 0 && !visited[i - 1][j] && grid[i - 1][j] == '1' {
                    q.push_back((i - 1, j));
                }
                if j + 1 < grid[0].len() && !visited[i][j + 1] && grid[i][j + 1] == '1' {
                    q.push_back((i, j + 1));
                }
                if j > 0 && !visited[i][j - 1] && grid[i][j - 1] == '1' {
                    q.push_back((i, j - 1));
                }
            }
            None => unreachable!(),
        }
    }
}

fn dfs_dp_grid(grid: &Vec<Vec<char>>, mut visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    visited[i][j] = true;
    if i < grid.len() - 1 && !visited[i + 1][j] && grid[i + 1][j] == '1' {
        dfs_dp_grid(&grid, &mut visited, i + 1, j);
    }
    if j < grid[0].len() - 1 && !visited[i][j + 1] && grid[i][j + 1] == '1' {
        dfs_dp_grid(&grid, &mut visited, i, j + 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ]),
            1
        );
    }
    #[test]
    fn test1() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands(grid), 3);
    }

    #[test]
    fn test2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands(grid), 3);
    }

    #[test]
    fn test3() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands(grid), 3);
    }

    #[test]
    fn basic_dp() {
        assert_eq!(
            num_islands_dp(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ]),
            1
        );
    }
    #[test]
    fn test1_dp() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands_dp(grid), 3);
    }

    #[test]
    fn test2_dp() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands_dp(grid), 3);
    }

    #[test]
    fn test3_dp() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands_dp(grid), 3);
    }
}
