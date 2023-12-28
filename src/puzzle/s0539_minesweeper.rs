pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
    let click = (click[0] as usize, click[1] as usize);
    match board[click.0][click.1] {
        'M' => {
            board[click.0][click.1] = 'X';
        }
        'E' => {
            let mut stack = Vec::new();
            stack.push(click);
            while !stack.is_empty() {
                let cur_pos = stack.pop().unwrap();
                let bomb_num = get_bomb_num(&board, (cur_pos.0 as i32, cur_pos.1 as i32));
                if bomb_num > 0 {
                    board[cur_pos.0][cur_pos.1] = (bomb_num as u8 + '0' as u8) as char;
                } else {
                    board[cur_pos.0][cur_pos.1] = 'B';
                    let (m, n) = (board.len() as i32, board[0].len() as i32);
                    for j in (cur_pos.0 as i32 - 1).max(0)..=(cur_pos.0 as i32 + 1).min(m - 1) {
                        for i in (cur_pos.1 as i32 - 1).max(0)..=(cur_pos.1 as i32 + 1).min(n - 1) {
                            if board[j as usize][i as usize] == 'E' {
                                stack.push((j as usize, i as usize));
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    };
    board
}

fn get_bomb_num(board: &Vec<Vec<char>>, pos: (i32, i32)) -> i32 {
    let mut num = 0;
    let (m, n) = (board.len() as i32, board[0].len() as i32);
    for j in (pos.0 - 1).max(0)..=(pos.0 + 1).min(m - 1) {
        for i in (pos.1 - 1).max(0)..=(pos.1 + 1).min(n - 1) {
            if (pos.0 != j || pos.1 != i) && board[j as usize][i as usize] == 'M' {
                num += 1;
            }
        }
    }
    num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn minesweeper_test() {
        let grid = vec![
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'M', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
        ];

        let resp = vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'M', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B'],
        ];
        assert_eq!(resp, update_board(grid, vec![3, 0]));
    }
    #[test]
    fn minesweeper_test_2() {
        let grid = vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'M', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B'],
        ];

        let resp = vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'X', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B'],
        ];
        assert_eq!(resp, update_board(grid, vec![1, 2]));
    }
}
