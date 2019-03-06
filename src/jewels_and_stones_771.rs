// @url https://leetcode.com/problems/jewels-and-stones/

#[allow(dead_code)]
pub fn num_jewels_in_stones_1(j: String, s: String) -> i32 {
    let (start, end) = ('A' as usize, 'z' as usize);
    let mut mark = vec![false; end-start+1];
    for ch in j.bytes() {
        mark[ch as usize - start] = true;
    }
    let mut count = 0;
    for ch in s.bytes() {
        if mark[ch as usize - start] == true { count += 1; }
    }
    count
}

#[allow(dead_code)]
pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let mut res = 0;
    for ch in s.chars() {
        if j.contains(ch) { res += 1; }
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let j = String::from("aA");
        let s = String::from("aAAbbbb");
        assert_eq!(num_jewels_in_stones(j, s), 3);
    }
    #[test]
    fn it_works_02() {
        let j = String::from("z");
        let s = String::from("ZZ");
        assert_eq!(num_jewels_in_stones(j, s), 0);
    }
}
