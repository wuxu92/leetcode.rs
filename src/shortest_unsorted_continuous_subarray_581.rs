// @url https://leetcode.com/problems/shortest-unsorted-continuous-subarray/

// Wronged
#[allow(dead_code)]
pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let (mut low, mut hi) = (nums.len(), 0);
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] < nums[j] {
                if j < low { low = j; }
                break;
            }
        }
        if low == 0 { break }
    }
    for i in (0..nums.len()-1).rev() {
        for j in (i+1..nums.len()).rev() {
            if nums[i] > nums[j] {
                if j > hi { hi = j; }
                break;
            }
        }
        if hi == nums.len()-1 { break }
    }
    (hi - low) as i32
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]), 5);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(find_unsorted_subarray(vec![2]), 0);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(find_unsorted_subarray(vec![2, 6]), 0);
    }
    #[test]
    fn it_works_04() {
        assert_eq!(find_unsorted_subarray(vec![6, 2, 6]), 2);
    }
    #[test]
    fn it_works_05() {
        assert_eq!(find_unsorted_subarray(vec![6, 2]), 2);
    }
    #[test]
    fn it_works_06() {
        assert_eq!(find_unsorted_subarray(vec![]), 0);
    }
}
