// @url https://leetcode.com/problems/sliding-window-maximum/
// You may assume k is always valid, 1 ≤ k ≤ input array's size for non-empty array

#[allow(dead_code)]
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    vec![]
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
}
