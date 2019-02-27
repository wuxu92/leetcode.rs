// @url https://leetcode.com/problems/top-k-frequent-elements/

// use std::sort
struct Pair {
    val: i32,
    cnt: usize,
}

impl Pair {
    #[allow(dead_code)]
    #[inline]
    pub fn new(v: i32, cnt: usize) -> Self {
        Pair{val: v, cnt: cnt }
    }
}

#[allow(dead_code)]
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res : Vec<i32> = vec![];
    use std::collections::BTreeMap;
    let mut map : BTreeMap<i32, usize> = BTreeMap::new();
    for &num in nums.iter() {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut vec = vec![];
    for (num, cnt) in map { vec.push(Pair::new(num, cnt)); }
    vec.sort_unstable_by(|x, y| {y.cnt.cmp(&x.cnt)});

    for i in 0..k as usize { res.push(vec[i].val); }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(top_k_frequent(vec![1,1,1,2,2,3], 2), vec![1,2]);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
