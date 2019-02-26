// @url https://leetcode.com/problems/longest-increasing-subsequence/

#[allow(dead_code)]
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0 }
    let mut dp : Vec<i32> = vec![1; nums.len()+1];
    let mut max;
    for i in 1..nums.len() {
        max = 1;
        let val = nums[i];
        for j in 0..i {
            if val > nums[j] && dp[j]+1 > max { max = dp[j]+1; }
        }
        dp[i] = max;
    }
    max = 0;
    for i in 0..nums.len() {
        if max < dp[i] { max = dp[i]; }
    }
    max
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(length_of_lis(vec![10,9,2,5,3,7,101,18]), 4);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(length_of_lis(vec![1,9,2,5,3,7,101,18]), 5);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(length_of_lis(vec![10,9,2]), 1);
    }
    #[test]
    fn it_works_04() {
        assert_eq!(length_of_lis(vec![10]), 1);
    }
    #[test]
    fn it_works_05() {
        assert_eq!(length_of_lis(vec![1,2,5,6,7,11,18]), 7);
    }
    #[test]
    fn it_works_06() {
        assert_eq!(length_of_lis(vec![1,3,6,7,9,4,10,5,6]), 6);
    }
}
