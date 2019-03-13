// @url https://leetcode.com/problems/minimum-window-substring/

#[allow(dead_code)]
pub fn min_window(s: String, t: String) -> String {
    if t.len() == 0 { return String::new() }
    let mut mark = vec![0;128];
    let bytes : Vec<_> = s.bytes().collect();
    for ch in t.bytes() {
        mark[ch as usize] += 1;
    }
    let (mut l, mut d, mut count) = (0, std::usize::MAX, t.len());
    let mut head=0;
    for r in 0..s.len() {
        let idx = bytes[r] as usize;
        if mark[idx] > 0 { count -= 1 };
        mark[idx] -= 1;
        while count == 0 {
            let d2 = r - l;
            if d2 < d {
                d = d2; head = l;
            }
            let ldx = bytes[l] as usize;
            if mark[ldx] == 0 { count += 1; }
            mark[ldx] += 1;
            l += 1;
        }
    }
    if d == std::usize::MAX { return String::new() }
    String::from(s.get(head..head+d+1).unwrap())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABC");
        let res = String::from("BANC");
        assert_eq!(min_window(s, t), res);
    }
    #[test]
    fn it_works_02() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("A");
        let res = String::from("A");
        assert_eq!(min_window(s, t), res);
    }
    #[test]
    fn it_works_03() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("");
        let res = String::from("");
        assert_eq!(min_window(s, t), res);
    }
    #[test]
    fn it_works_04() {
        let s = String::from("AD");
        let t = String::from("D");
        let res = String::from("D");
        assert_eq!(min_window(s, t), res);
    }
    #[test]
    fn it_works_05() {
        let s = String::from("AD");
        let t = String::from("AD");
        let res = String::from("AD");
        assert_eq!(min_window(s, t), res);
    }
    #[test]
    fn it_works_06() {
        let s = String::from("bdab");
        let t = String::from("ab");
        let res = String::from("ab");
        assert_eq!(min_window(s, t), res);
    }
}
