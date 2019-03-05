// @url https://leetcode.com/problems/find-all-anagrams-in-a-string/

#[allow(dead_code)]
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut res : Vec<i32> = vec![];
    if p.len() >= s.len() && s != p { return res }
    // loop
    let mut pmark : Vec<i32> = vec![0; 32];
    let mut smark = pmark.clone();
    let a_num = 'a' as u8;
    for ch in p.bytes() {
        pmark[(ch - a_num) as usize] += 1;
    }
    let plen = p.len();
    let sb : Vec<u8> = s.bytes().collect();
    for idx in 0..plen {
        smark[(sb[idx] - a_num) as usize] += 1;
    }
    if pmark == smark { res.push(0); }
    for i in 0..s.len()-plen {
        smark[(sb[i]-a_num) as usize] -= 1;
        smark[(sb[i+plen] - a_num) as usize] += 1;
        if pmark == smark { res.push(i as i32 + 1); }
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let s = String::from("cbaebabacd");
        let p = String::from("abc");
        let res = vec![0, 6];
        assert_eq!(find_anagrams(s, p), res);
    }
    #[test]
    fn it_works_02() {
        let s = String::from("abab");
        let p = String::from("ab");
        let res = vec![0, 1, 2];
        assert_eq!(find_anagrams(s, p), res);
    }
    #[test]
    fn it_works_03() {
        let s = String::from("aaa");
        let p = String::from("a");
        let res = vec![0, 1, 2];
        assert_eq!(find_anagrams(s, p), res);
    }
}
