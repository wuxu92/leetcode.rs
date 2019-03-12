// @url https://leetcode.com/problems/sliding-window-maximum/
// You may assume k is always valid, 1 ≤ k ≤ input array's size for non-empty array

#[allow(dead_code)]
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.len() == 0 || k == 0 { return vec![] }
    let (mut res, len) = (nums.clone(), nums.len());
    for i in 0..k as usize - 1 {
        for j in 0..len-i-1 {
            if res[j] < res[j+1] { res[j] = res[j+1]; }
        }
    }
    for _i in 0..k-1 {
        res.pop();
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(
            max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3),
            vec![3,3,5,5,6,7]
        );
    }
    #[test]
    fn it_works_02() {
        assert_eq!(
            max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 1),
            vec![1,3,-1,-3,5,3,6,7]
        );
    }
    #[test]
    fn it_works_03() {
        assert_eq!(
            max_sliding_window(vec![1,3,-1], 3),
            vec![3]
        );
    }
    #[test]
    fn it_works_04() {
        assert_eq!(
            max_sliding_window(vec![], 0),
            vec![]
        );
    }
}
