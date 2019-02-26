// @url https://leetcode.com/problems/longest-increasing-subsequence/

// simple dp
#[allow(dead_code)]
pub fn length_of_lis_01(nums: Vec<i32>) -> i32 {
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

#[allow(dead_code)]
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut sv : Vec<i32> = vec![];
    sv.reserve(nums.len());

    for num in nums {
        if sv.len() == 0 { sv.push(num); continue }
        if num > sv[sv.len()-1] { sv.push(num); continue }
        // search
        let (mut lo, mut hi) = (0, sv.len()-1);
        while lo < hi {
            let mid = (lo+hi)/2;
            let v = sv[mid];
            if v == num { lo = mid;  break }
            if v < num { lo = mid+1; }
            else { hi = mid; }
        }
        sv[lo] = num;
    }
    sv.len() as i32
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
    #[test]
    fn it_works_07() {
        assert_eq!(length_of_lis(vec![]), 0);
    }
    #[test]
    fn it_works_08() {
        assert_eq!(length_of_lis(vec![3,5,6,2,5,4,19,5,6,7,12]), 6);
    }
}
