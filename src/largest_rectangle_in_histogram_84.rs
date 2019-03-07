// @url https://leetcode.com/problems/largest-rectangle-in-histogram/

#[allow(dead_code)]
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let (mut res, mut pre, h) = (0, 0, heights);
    for idx in 0..h.len() {
        let cur = h[idx];
        if cur == pre { continue }
        pre = cur;
        // find pre—idx and post-idx
        let (mut pre_idx, mut post_idx) = (idx, idx);
        for i in (0..idx).rev() {
            if h[i] < cur { break; }
            pre_idx = i;
        }
        for i in idx+1..h.len() {
            if h[i] < cur { break; }
            post_idx = i;
        }
        let sum = cur * (post_idx - pre_idx + 1) as i32;
        if sum > res { res = sum; }
    }
    res
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
