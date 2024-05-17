pub struct Solution {}

// problem: https://leetcode.com/problems/rotate-image/
// discuss: https://leetcode.com/problems/rotate-image/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// x,y ->  y,n-x     2-dimension vector rotate -90 degree:
//   ^                  x     0   1      y
//   |      |              *         =
//          v           y     -1  0      -x
// n-y,x <- n-x,n-y  if we consider axis transform, then: rotate(x, y) = (y, -x + n)

//  we only need to iterate a 1/4 corner matrix, for odd matrix, we take an extra part in x direction
//
//  even:
//
//  x x o o
//  x x o o
//  o o o o
//  o o o o
//
//  odd:
//
//  x x o o o
//  x x o o o
//  x x o o o
//  o o o o o
//  o o o o o

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut matrix = matrix;
        let (len, n) = (matrix.len(), matrix.len() - 1);
        for x in 0..len / 2 {
            for y in 0..(len + 1) / 2 {
                let temp = matrix[x][y];
                matrix[x][y] = matrix[n - y][x];
                matrix[n - y][x] = matrix[n - x][n - y];
                matrix[n - x][n - y] = matrix[y][n - x];
                matrix[y][n - x] = temp;
            }
        }
    }
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let mut dimension = matrix.len();
    let mut offset = 0;
    while dimension > 1 {
        let buflen = matrix.len() - 1 - offset * 2;
        let mut buf: Vec<i32> = Vec::with_capacity(buflen);
        for i in 0..buflen {
            buf.push(matrix[offset][offset + i]);
        }
        println!("buf = {:#?}", buf);
        // rotate
        for i in 0..buflen {
            matrix[offset + 0][offset + i] = matrix[offset + buflen - i][offset + 0];
            matrix[offset + buflen - i][offset + 0] = matrix[offset + buflen][offset + buflen - i];
            matrix[offset + buflen][offset + buflen - i] = matrix[offset + i][offset + buflen];
            matrix[offset + i][offset + buflen] = buf[i];
        }
        dimension -= 2;
        offset += 1;
        println!("matrix = {:?}", matrix);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_48() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }

    #[test]
    fn test_48_s() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
