// @url https://leetcode.com/problems/word-break/

// may exceeded time limit
#[allow(dead_code)]
pub fn word_break_1(s: String, word_dict: Vec<String>) -> bool {
    fn process(s: &str, dict: &Vec<String>) -> bool {
        for w in dict {
            if s.starts_with(w) {
                if s == w { return true }
                if process(&s[w.len()..], dict) { return true }
            }
        }
        false
    }
    return process(s.as_str(), &word_dict);
}

// may till exceeded time limit
#[allow(dead_code)]
pub fn word_break_02(s: String, word_dict: Vec<String>) -> bool {
    fn process(s: &str, dict: &Vec<String>) -> bool {
        for w in dict.iter() {
            if ! s.starts_with(w) { continue }
            if s == w { return true }
            for w2 in dict.iter() {
                if ! s.ends_with(w2) { continue }
                if s == w2 || s == format!("{}{}", w, w2) { return true }
                if w.len() + w2.len() > s.len() { continue; }
                let ends = s.len() - w2.len();
                if process(&s[w.len()..ends], dict) { return true }
            }
        }
        false
    }
    return process(s.as_str(), &word_dict);
}

#[allow(dead_code)]
pub fn word_break_03(s: String, word_dict: Vec<String>) -> bool {
    // process pure contains
    if word_dict.len() == 0 { return false };
    let mut dicts = word_dict.clone();
    dicts.sort_unstable_by(|a, b| {a.len().cmp(&b.len()).reverse()});
    let mut marker = vec![false; s.len()*s.len()+3];

    fn process(s: &str, marker: &mut Vec<bool>, dict: &Vec<String>, b: usize, e: usize) -> bool {
        let len = s.len();
        if marker[b*len+e] { return false };
        for w in dict.iter() {
            if ! s.starts_with(w) { continue }
            if s == w { return true }
            for w2 in dict.iter() {
                if ! s.ends_with(w2) { continue }
                if s == w2 || s == format!("{}{}", w, w2) { return true }
                if w.len() + w2.len() > len { continue; }
                let (begin, ends) = (w.len(), len - w2.len());
                if marker[begin*len+ends] { continue }
                if process(&s[begin..ends], marker,  dict, begin, ends) { return true }
                marker[begin*len+ends] = true;
            }
        }
        false
    }
    return process(s.as_str(), &mut marker, &word_dict, 0, s.len());
}

// use only start prune
#[allow(dead_code)]
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    // process pure contains
    if word_dict.len() == 0 { return false };
    let mut dicts = word_dict.clone();
    dicts.sort_unstable_by(|a, b| {a.len().cmp(&b.len()).reverse()});
    let mut marker = vec![false; s.len()+3];

    fn process(s: &str, marker: &mut Vec<bool>, dict: &Vec<String>, b: usize ) -> bool {
        if marker[b] { return false };
        for w in dict.iter() {
            if s == w { return true }
            let idx = b + w.len();
            if s.len() < w.len() || marker[idx] ||
                ! s.starts_with(w) { continue }
            if process(&s[w.len()..], marker, dict, idx) { return true }
            marker[idx] = true;
        }
        false
    }
    return process(s.as_str(), &mut marker, &word_dict, 0);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    pub fn s(ss: &str) -> String { String::from(ss) }
    pub fn ss(vec: Vec<&str>) -> Vec<String> {
        let mut v : Vec<String> = vec![];
        for s in vec {
            v.push(String::from(s));
        }
        return v
    }
    #[test]
    fn it_works_01() {
        assert_eq!(word_break(
                String::from("leetcode"),
                ss(vec!["leet","cod","e"])
                ), true);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(word_break(
                String::from("applepenapple"),
                ss(vec!["apple", "pen"])
                ), true);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(word_break(
                String::from("catsandog"),
                ss(vec!["cats", "dog", "sand", "and", "cat"])
                ), false);
    }
    #[test]
    fn it_works_04() {
            
        assert_eq!(word_break(
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"),
                ss(vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"])
                ), false);
    }
    #[test]
    fn it_works_05() {
            
        assert_eq!(word_break(
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                ss(vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"])
                ), false);
    }
    #[test]
    fn it_works_06() {
        assert_eq!(word_break(
                String::from("a"),
                ss(vec![])
                ), false);
    }
    #[test]
    fn it_works_07() {
        assert_eq!(word_break(
                String::from(""),
                ss(vec![])
                ), false);
    }
    #[test]
    fn it_works_08() {
        assert_eq!(word_break(
                String::from(""),
                ss(vec!["a", ""])
                ), true);
    }
    #[test]
    fn it_works_09() {
        assert_eq!(word_break(
                String::from("a"),
                ss(vec!["a"])
                ), true);
    }
}
