pub fn min_remove_to_make_valid(s: String) -> String {
    let mut letters = Vec::with_capacity(s.len());
    let mut stack = Vec::new();

    s.chars().for_each(|c| match c {
        '(' => {
            letters.push('#');
            stack.push((letters.len() - 1, c));
        }
        ')' => {
            if let Some((pos, letter)) = stack.pop() {
                letters[pos] = letter;
                letters.push(c);
            }
        }
        _ => letters.push(c),
    });

    letters.into_iter().filter(|&c| c != '#').collect()
}

struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(mut s: String) -> String {
        let mut curr_open_count = 0;
        let mut remaining_close_count = s.chars().filter(|c| *c == ')').count();

        s.retain(|c| match c {
            '(' => {
                curr_open_count += 1;
                if remaining_close_count >= curr_open_count {
                    true
                } else {
                    false
                }
            }
            ')' => {
                remaining_close_count -= 1;
                if curr_open_count > 0 {
                    curr_open_count -= 1;
                    true
                } else {
                    false
                }
            }
            _ => true,
        });

        s
    }
}

pub fn min_remove_to_make_valid_zero_alloc(mut s: String) -> String {
    let mut depth: usize = 0;
    s.retain(|c| match c {
        '(' => {
            depth += 1;
            true
        }
        ')' => {
            let d = depth.checked_sub(1);
            depth = d.unwrap_or(0);
            d.is_some()
        }
        _ => true,
    });

    if depth > 0 {
        let mut valid_until: usize = s.len();
        for (i, &c) in s.as_bytes().iter().enumerate() {
            if depth == 0 {
                break;
            }
            if c == b'(' {
                valid_until = i;
                depth -= 1;
            }
        }
        let mut i: usize = 0;
        s.retain(|c| {
            let drop = i >= valid_until && c == '(';
            i += c.len_utf8();
            !drop
        });
    }
    s
}
