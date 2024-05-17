use std::cmp::max;
use std::collections::VecDeque;

const DXY: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn max_area_of_islands(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut max_area: i32 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                max_area = max(max_area, island_area(&mut grid, (i as i32, j as i32)));
            }
        }
    }
    max_area
}

fn island_area(grid: &mut Vec<Vec<i32>>, pos: (i32, i32)) -> i32 {
    if grid[pos.0 as usize][pos.1 as usize] == 0 {
        0
    } else {
        let mut area = 0;
        let mut que = VecDeque::new();
        let (m, n) = (grid.len(), grid[0].len());
        grid[pos.0 as usize][pos.1 as usize] = 0;
        que.push_back(pos);
        while !que.is_empty() {
            let last_pos = que.pop_front().unwrap();
            area += 1;
            for &d in &DXY {
                let next_pos: (i32, i32) = (last_pos.0 + d.0, last_pos.1 + d.1);
                if next_pos.0 >= 0
                    && next_pos.0 < m as i32
                    && next_pos.1 >= 0
                    && next_pos.1 < n as i32
                    && grid[next_pos.0 as usize][next_pos.1 as usize] == 1
                {
                    grid[next_pos.0 as usize][next_pos.1 as usize] = 0;
                    que.push_back(next_pos);
                }
            }
        }
        area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn max_area_island_test_1() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];

        assert_eq!(6, max_area_of_islands(grid));
    }

    #[test]
    fn max_area_island_test_2() {
        let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(0, max_area_of_islands(grid))
    }
}
