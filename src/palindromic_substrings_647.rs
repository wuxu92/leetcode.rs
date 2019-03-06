// @url https://leetcode.com/problems/palindromic-substrings/

// expand from i
#[allow(dead_code)]
pub fn count_substrings(s: String) -> i32 {
    let chars : Vec<char> = s.chars().collect();
    let (mut count, len) = (0, chars.len() );
    for idx in 0..len {
        count += 1;
        let wide = if idx < len-idx-1 { idx } else { len-idx-1 };
        for i in 1..wide+1 {
            if chars[idx-i] == chars[idx+i] { count += 1; }
            else { break }
        }
        let (mut dec, mut inc) = (0, 1);
        while idx + inc < len {
            if chars[idx-dec] != chars[idx+inc] { break }
            count += 1;
            if idx == dec { break; }
            dec += 1; inc += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let s = String::from("abc");
        assert_eq!(count_substrings(s), 3);
    }
    #[test]
    fn it_works_02() {
        let s = String::from("aaa");
        assert_eq!(count_substrings(s), 6);
    }
    #[test]
    fn it_works_03() {
        let s = String::from("a");
        assert_eq!(count_substrings(s), 1);
    }
    #[test]
    fn it_works_04() {
        let s = String::from("aaaaa");
        assert_eq!(count_substrings(s), 15);
    }
    #[test]
    fn it_works_05() {
        let s = String::from("fdsklf");
        assert_eq!(count_substrings(s), 6);
    }
}
