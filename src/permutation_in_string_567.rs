// @url https://leetcode.com/problems/permutation-in-string/

//  if s2 contains the permutation of s1.
#[allow(dead_code)]
pub fn check_inclusion_1(s1: String, s2: String) -> bool {
    let len1 = s1.len();
    if s2.len() < s1.len() { return false }
    if len1 == 0 { return true }

    let byte1 : Vec<_> = s1.bytes().collect();
    let mut map : Vec<usize> = vec![0; 128];
    for &b in byte1.iter() { map[b as usize] += 1; }

    let mut count = len1;
    let bytes : Vec<_> = s2.bytes().collect();
    for r in 0..bytes.len() {
        let idx = bytes[r] as usize;
        if map[idx] == 0 {
            if count != len1 {
                // reverse
                if let Some(slice) = bytes.get((r-(len1-count))..r) {
                    for &ch in slice {
                        map[ch as usize] += 1;
                        count += 1;
                        if map[idx] > 0 { break; }
                    }
                }
            }
        }
        if map[idx] > 0 { map[idx] -= 1; count -= 1; }
        if count == 0 { return true }
    }
    false
}

// slide window
#[allow(dead_code)]
pub fn check_inclusion(s1: String, s2: String) -> bool {
    let len1 = s1.len();
    if len1 > s2.len() { return false }
    let byte1 : Vec<_> = s1.bytes().collect();
    let byte2 : Vec<_> = s2.bytes().collect();
    let mut map : Vec<i32> = vec![0; 26];
    let anum = 'a' as u8;
    for i in 0..len1 {
        map[(byte1[i] - anum) as usize] += 1;
        map[(byte2[i] - anum) as usize] -= 1;
    }
    fn check_zero(m: &Vec<i32>) -> bool {
        for n in m { if *n != 0 { return false } }
        return true
    }
    if check_zero(&map) { return true }
    for i in len1..byte2.len() {
        map[(byte2[i-len1] - anum) as usize] += 1;
        map[(byte2[i] - anum) as usize] -= 1;
        if check_zero(&map) { return true }
    }
    false
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let s1 = String::from("ab");
        let s2 = String::from("eidbaooo");
        assert_eq!(check_inclusion(s1, s2), true);
    }
    #[test]
    fn it_works_02() {
        let s1 = String::from("ab");
        let s2 = String::from("eidboaoo");
        assert_eq!(check_inclusion(s1, s2), false);
    }
    #[test]
    fn it_works_03() {
        let s1 = String::from("b");
        let s2 = String::from("eidboaoo");
        assert_eq!(check_inclusion(s1, s2), true);
    }
    #[test]
    fn it_works_04() {
        let s1 = String::from("");
        let s2 = String::from("e");
        assert_eq!(check_inclusion(s1, s2), true);
    }
    #[test]
    fn it_works_05() {
        let s1 = String::from("ab");
        let s2 = String::from("eidbaoaoo");
        assert_eq!(check_inclusion(s1, s2), true);
    }
    #[test]
    fn it_works_06() {
        let s1 = String::from("adc");
        let s2 = String::from("dcda");
        assert_eq!(check_inclusion(s1, s2), true);
    }
    #[test]
    fn it_works_07() {
        let s1 = String::from("hello");
        let s2 = String::from("ooolleoooleh");
        assert_eq!(check_inclusion(s1, s2), false);
    }
    #[test]
    fn it_works_08() {
        let s1 = String::from("ab");
        let s2 = String::from("a");
        assert_eq!(check_inclusion(s1, s2), false);
    }
    #[test]
    fn it_works_09() {
        let s1 = String::from("ab");
        let s2 = String::from("ab");
        assert_eq!(check_inclusion(s1, s2), true);
    }
}
