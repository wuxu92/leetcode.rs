// url: https://leetcode.com/problems/group-anagrams/

pub fn group_anagrams_1(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut res : Vec<Vec<String>> = Vec::new();
    // build a map to store it
    let mut map : HashMap<String, Vec<String>> = HashMap::new();
    for (_idx, s) in strs.iter().enumerate() {
        let mut key : Vec<char> = s.chars().collect();
        key.sort();
        let key:String = key.iter().collect();
        if ! map.contains_key(&key) {
            map.insert(key.to_string(), vec![s.clone()]);
        } else if let Some(v) = map.get_mut(&key) { v.push(s.clone()); }
    }
    // loop map
    for (_key, vec) in map.iter() {
        res.push(vec.clone());
    }
    res
}

// no sort
pub fn group_anagrams_2(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::{HashMap, BTreeMap};

    let mut res : Vec<Vec<String>> = Vec::new();
    let mut map : HashMap<String, Vec<String>> = HashMap::new();
    for s in strs.iter() {
        let mut chs : BTreeMap<char, u8> = BTreeMap::new();
        for ch in s.chars() {
            let count = chs.entry(ch).or_insert(0);
            *count += 1;
        }
        let mut key = String::new();
        for (k, v) in chs.iter() {
            key.push(*k);
            key.push(*v as char);
        }
        let mut vec = map.entry(key).or_insert(Vec::new());
        vec.push(s.clone());
    }
    for (_key, vec) in map.iter() {
        res.push(vec.clone());
    }
    res
}

// no btree
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut res : Vec<Vec<String>> = Vec::new();
    let mut map : HashMap<String, Vec<String>> = HashMap::new();
    let a_num = 'a' as u8;
    for s in strs {
        let mut counter : [u8; 26] = [0; 26];
        for ch in s.chars() {
            counter[(ch as u8  - a_num) as usize] += 1;
        }
        let mut key = String::new();
        for i in 0..26 {
            if counter[i] > 0 {
                key.push((i as u8 +a_num) as char);
                key.push(counter[i] as char);
            }
        }
        let mut vec = map.entry(key).or_insert(Vec::new());
        vec.push(s.clone());
    }
    for vec in map.values() {
        res.push(vec.clone());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()]), vec![
                   vec!["bat"],
                   vec!["tan", "nat"],
                   vec!["eat", "tea", "ate"]
        ]);
    }
}
