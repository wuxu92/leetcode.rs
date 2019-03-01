// @url https://leetcode.com/problems/decode-string/

#[allow(dead_code)]
pub fn decode_string(s: String) -> String {
    let chars : Vec<char> = s.chars().collect();

    fn process(s: &[char], idx: &mut usize) -> String {
        let mut res = String::new();
        while *idx < s.len() {
            let ch = s[*idx];
            if ch == ']' { *idx += 1; break }
            if ch != '[' { res.push(ch); *idx += 1; continue; }
            // if ch '[', find last number
            let (mut multi, mut base) = (0 as usize, 1);
            while let Some(num) = res.pop() {
                if num >= '0' && num <= '9' {
                    multi = (num as u8 - '0' as u8) as usize * base + multi;
                    base *= 10;
                } else {
                    res.push(num);  // push back if not number
                    break;
                }
            }
            *idx += 1;
            let sub_str = process(s, idx);
            for _ in 0..multi{
                res.push_str(sub_str.as_str());
            }
        }
        res
    }
    let mut idx = 0;
    process(&mut chars.as_slice(), &mut idx)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(decode_string(String::from("3[a]2[bc]")), String::from("aaabcbc"));
    }
    #[test]
    fn it_works_02() {
        let s1 = String::from("3[a2[c]]");
        let s2 = String::from("accaccacc");
        assert_eq!(decode_string(s1), s2);
    }
    #[test]
    fn it_works_03() {
        let s1 = String::from("2[abc]3[cd]ef");
        let s2 = String::from("abcabccdcdcdef");
        assert_eq!(decode_string(s1), s2);
    }
    #[test]
    fn it_works_04() {
        let s1 = String::from("2");
        let s2 = String::from("2");
        assert_eq!(decode_string(s1), s2);
    }
    #[test]
    fn it_works_05() {
        let s1 = String::from("2[]");
        let s2 = String::from("");
        assert_eq!(decode_string(s1), s2);
    }
    #[test]
    fn it_works_06() {
        let s1 = String::from("100[leetcode]");
        let s2 = String::from("leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode");
        assert_eq!(decode_string(s1), s2);
    }
}
