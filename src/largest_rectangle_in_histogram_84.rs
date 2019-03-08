// @url https://leetcode.com/problems/largest-rectangle-in-histogram/

// intuitive solution, simulate
#[allow(dead_code)]
pub fn largest_rectangle_area_1(heights: Vec<i32>) -> i32 {
    let (mut res, mut pre, h) = (0, 0, heights);
    for idx in 0..h.len() {
        let cur = h[idx];
        if cur == pre { continue }
        pre = cur;
        // find pre—idx and post-idx
        let (mut pre_idx, mut post_idx) = (idx, idx);
        let mut has_pre = false;
        for i in (0..idx).rev() {
            if h[i] < cur { break; }
            if h[i] == cur { has_pre = true; break }
            pre_idx = i;
        }
        if has_pre { continue; }
        for i in idx+1..h.len() {
            if h[i] < cur { break; }
            post_idx = i;
        }
        let sum = cur * (post_idx - pre_idx + 1) as i32;
        if sum > res { res = sum; }
    }
    res
}

// use a map, Even much slower than the first solution
#[allow(dead_code)]
pub fn largest_rectangle_area_2(heights: Vec<i32>) -> i32 {
    let (mut res, h) = (0, heights);
    use std::collections::BTreeMap;
    let mut map : BTreeMap<i32, usize> = BTreeMap::new();
    for idx in 0..h.len() {
        let cur = h[idx];
        if ! map.contains_key(&cur) {
            let mut pre_start = idx;
            for (&key, &idx) in map.iter() {
                if key >= cur { pre_start = idx; break; }
            }
            let (mut pre_idx, mut post_idx) = (pre_start, idx);
            for i in (0..pre_start).rev() {
                if h[i] < cur { break; } else { pre_idx = i; }
            }
            for i in idx+1..h.len() {
                if h[i] < cur { break; } else { post_idx = i; }
            }
            let sum = cur * (post_idx - pre_idx + 1) as i32;
            if sum > res { res = sum; }
            map.insert(cur, idx);
        }
        let keys : Vec<_> = map.keys().cloned().collect();
        for &k in keys.iter().rev() {
            if k <= cur { break }
            map.remove(&k);
        }
    }
    res
}

// @see https://www.geeksforgeeks.org/largest-rectangle-under-histogram/
#[allow(dead_code)]
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack : Vec<usize> = vec![];
    let (mut max, mut h) = (0, heights);
    h.push(0);
    let mut idx = 0;
    while idx < h.len() {
        // println!("idx: {} {}, maxl: {} s: {:?}", idx, h[idx], max, stack);
        let len = stack.len();
        if len == 0 || h[idx] > h[stack[len-1]] {
            stack.push(idx); idx += 1; continue
        }
        let len = len - 1;
        let top = stack[len];
        let ll = if len > 0 { idx - 1 - stack[len-1] } else { idx };
        let sum = h[top] * ll as i32;
        if sum > max { max = sum; }
        stack.pop();
    }
    max
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![2,1,5,6,2,3];
        assert_eq!(largest_rectangle_area(ins), 10);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![2,1,5,2,3];
        assert_eq!(largest_rectangle_area(ins), 6);
    }
    #[test]
    fn it_works_03() {
        let ins = vec![3,1];
        assert_eq!(largest_rectangle_area(ins), 3);
    }
    #[test]
    fn it_works_04() {
        let ins = vec![3];
        assert_eq!(largest_rectangle_area(ins), 3);
    }
    #[test]
    fn it_works_05() {
        let ins = vec![0];
        assert_eq!(largest_rectangle_area(ins), 0);
    }
    #[test]
    fn it_works_06() {
        let ins = vec![];
        assert_eq!(largest_rectangle_area(ins), 0);
    }
}
