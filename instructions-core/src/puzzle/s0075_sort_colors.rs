pub fn sort_colors(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return;
    }

    let (mut lower_idx, mut upper_idx) = (0_usize, nums.len() - 1);
    let mut i = 0_usize;
    while i <= upper_idx {
        if nums[i] < 1 {
            // lower_idx <= i, we've scanned it so we know nums[lower_idx] <= 1, i++
            nums.swap(lower_idx, i);
            i += 1;
            lower_idx += 1;
        } else if nums[i] > 1 {
            nums.swap(upper_idx, i);
            if upper_idx < 1 {
                break;
            }
            upper_idx -= 1;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }
}
