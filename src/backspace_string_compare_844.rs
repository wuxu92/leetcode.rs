// @url https://leetcode.com/problems/backspace-string-compare/

#[allow(dead_code)]
pub fn backspace_compare_1(s: String, t: String) -> bool {
    let mut ss = String::new();
    let mut st = String::new();
    for ch in s.bytes() {
        if ch == '#' as u8 {ss.pop();}
        else { ss.push(ch as char); }
    }
    for ch in t.bytes() {
        if ch == '#' as u8 {st.pop();}
        else { st.push(ch as char); }
    }
    ss == st
}

// one loop
#[allow(dead_code)]
pub fn backspace_compare(s: String, t: String) -> bool {
    let (mut cs, mut ct) = ('a', 'b');
    let (mut is, mut it) = (0, 0);
    let (mut bs, mut bt) = (s.chars().rev(), t.chars().rev());
    let (mut es, mut et) = (false, false);
    loop {
        loop {
            match bs.next() {
                None => { es = true; cs = 0 as char; break; },
                Some(ch) => {
                    if ch == '#' { is += 1; continue; }
                    if is == 0 { cs = ch; break; }
                    is -= 1;
                }
            };
        }
        loop {
            match bt.next() {
                None => { et = true; ct = 0 as char; break; },
                Some(ch) => {
                    if ch == '#' { it += 1; continue; }
                    if it == 0 { ct = ch; break; }
                    it -= 1;
                }
            };
        }
        if es && et { return true }
        if cs != ct { return false }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(backspace_compare(String::from("ab#c"), String::from("ad#c")), true);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(backspace_compare(String::from("a##c"), String::from("#a#c")), true);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(backspace_compare(String::from("a#c"), String::from("b")), false);
    }
    #[test]
    fn it_works_04() {
        assert_eq!(backspace_compare(String::from("ab##"), String::from("c#d#")), true);
    }
    #[test]
    fn it_works_05() {
        assert_eq!(backspace_compare(String::from("bbbextm"), String::from("bbb#extm")), false);
    }
}
