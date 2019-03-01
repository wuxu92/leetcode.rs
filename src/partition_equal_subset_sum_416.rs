// @url https://leetcode.com/problems/partition-equal-subset-sum/

// tanxin
#[allow(dead_code)]
pub fn can_partition(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    nums.sort_unstable();
    let sum : i32 = nums.iter().sum();
    let sum = sum as usize;
    if sum % 2 != 0 { return false }
    let mut dp : Vec<bool> = vec![false; sum as usize + 1];
    dp[0] = true;
    let mut max = 1;
    for num in nums {
        for idx in (0..max).rev() {
            if dp[idx] { dp[idx + num as usize] = true; }
        }
        max += num as usize;
    }
    dp[sum/2]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![1, 5, 11, 5];
        assert_eq!(can_partition(ins), true);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![1, 2, 3, 5];
        assert_eq!(can_partition(ins), false);
    }
    #[test]
    fn it_works_03() {
        let ins = vec![2, 2, 3, 5];
        assert_eq!(can_partition(ins), false);
    }
}
