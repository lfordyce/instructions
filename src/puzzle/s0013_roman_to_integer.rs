/**
 * [13] Roman to Integer
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 *
 *
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 *
 * For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.
 *
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 *
 * 	I can be placed before V (5) and X (10) to make 4 and 9.
 * 	X can be placed before L (50) and C (100) to make 40 and 90.
 * 	C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 *
 * Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.
 *
 * Example 1:
 *
 *
 * Input: "III"
 * Output: 3
 *
 * Example 2:
 *
 *
 * Input: "IV"
 * Output: 4
 *
 * Example 3:
 *
 *
 * Input: "IX"
 * Output: 9
 *
 * Example 4:
 *
 *
 * Input: "LVIII"
 * Output: 58
 * Explanation: L = 50, V= 5, III = 3.
 *
 *
 * Example 5:
 *
 *
 * Input: "MCMXCIV"
 * Output: 1994
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 *
 */
pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut acc = 0;
        let mut stream = s.chars().peekable();
        while let Some(curr) = stream.next() {
            let next = stream.peek();
            acc += match (curr, next) {
                ('I', Some('V')) => -1,
                ('I', Some('X')) => -1,
                ('X', Some('L')) => -10,
                ('X', Some('C')) => -10,
                ('C', Some('D')) => -100,
                ('C', Some('M')) => -100,
                ('I', _) => 1,
                ('V', _) => 5,
                ('X', _) => 10,
                ('L', _) => 50,
                ('C', _) => 100,
                ('D', _) => 500,
                ('M', _) => 1000,
                _ => panic!("Bad symbol")
            }
        }
        acc
    }
}

#[cfg(test)]
mod test {
   use super::*;

    #[test]
    fn test_13() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
    }

}