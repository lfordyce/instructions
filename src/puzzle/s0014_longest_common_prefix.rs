use std::str::Chars;

/**
 * [14] Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 * Example 1:
 *
 *
 * Input: ["flower","flow","flight"]
 * Output: "fl"
 *
 *
 * Example 2:
 *
 *
 * Input: ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *
 * Note:
 *
 * All given inputs are in lowercase letters a-z.
 *
 */

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut ret = "".to_string();
    let mut index = 0;
    if strs.len() == 0 || strs[0].len() == 0 {
        return ret;
    }
    let mut found = false;
    loop {
        ret.push_str(&strs[0][index..index + 1]);
        for i in 0..strs.len() {
            if strs[i].len() < index + 1 || strs[i][0..index + 1] != ret {
                found = true;
                break;
            }
        }
        match found {
            true => break ret[0..index].to_string(),
            false => {
                if index + 1 == strs[0].len() {
                    break ret;
                }
            }
        }
        index += 1;
    }
}

pub fn longest_common_prefix_alt(strs: Vec<String>) -> String {
    if strs.len() == 0 || strs[0].len() == 0 {
        return "".to_string();
    }
    let mut index = 0;
    for i in 0..strs[0].len() {
        let ch = strs[0].chars().nth(i);
        for str in &strs {
            if let Some(x) = str.chars().nth(i) {
                if ch != Some(x) {
                    return strs[0][..i].to_string();
                }
            } else {
                return strs[0][..i].to_string();
            }
        }
        index = i
    }
    strs[0][..=index].to_string()
}

pub fn longest_common_prefix_again(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    let mut iters: Vec<Chars> = strs.iter().map(|s| s.chars()).collect();
    let mut curr_char: Option<char> = None;
    if strs.len() < 1 {
        return prefix;
    }
    loop {
        curr_char.take().map(|ch| prefix.push(ch));
        for iter in iters.iter_mut() {
            let mut ch = iter.next();
            if ch.is_none() {
                return prefix;
            }
            match curr_char {
                None => curr_char = ch.take(),
                Some(curr) => {
                    if curr != ch.unwrap() {
                        return prefix;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            "fl".to_string(),
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
    }

    #[test]
    fn test_longest_common_prefix_alt() {
        assert_eq!(
            "fl".to_string(),
            longest_common_prefix_alt(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
        assert_eq!(
            "fl".to_string(),
            longest_common_prefix_alt(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
        assert_eq!(
            "".to_string(),
            longest_common_prefix_alt(vec![
                "racecar".to_string(),
                "flight".to_string(),
                "".to_string()
            ])
        );
    }

    #[test]
    fn test_14() {
        assert_eq!(
            longest_common_prefix_again(vec![
                "".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            longest_common_prefix_again(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(longest_common_prefix_again(vec![]), "");
    }
}
