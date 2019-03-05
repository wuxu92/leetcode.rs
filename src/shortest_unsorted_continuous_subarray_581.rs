// @url https://leetcode.com/problems/shortest-unsorted-continuous-subarray/

// Wronged
#[allow(dead_code)]
pub fn find_unsorted_subarray_err_1(nums: Vec<i32>) -> i32 {
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

// find after in-creasing mininum number and the max after decreasing from last
// two-passs solution
#[allow(dead_code)]
pub fn find_unsorted_subarray_1(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 { return 0 }
    let (mut lo, mut hi) = (0, 0);
    // find the lowest after increasing
    for idx in 1..nums.len() {
        if nums[idx] < nums[idx-1] { lo = idx; break; }
    }
    if lo == 0 { return 0 }
    let mut flag = std::i32::MAX;
    for idx in lo..nums.len() {
        if nums[idx] < flag { flag = nums[idx]; }
    }
    // find the pos in increasing
    for idx in 0..nums.len() {
        if nums[idx] > flag { lo = idx; break; }
    }
    // find the max
    flag = std::i32::MIN;
    for idx in (0..nums.len()-1).rev() {
        if nums[idx] > nums[idx+1] { hi = idx; break; }
    }
    // if hi == 0 { return 0 }
    for idx in (0..hi+1).rev() {
        if nums[idx] > flag { flag = nums[idx]; }
    }
    // find the pos in decresing
    for idx in (0..nums.len()).rev() {
        if nums[idx] < flag { hi = idx; break; }
    }
    (hi - lo + 1) as i32
}

// one-passs solution
#[allow(dead_code)]
pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let (mut hi, mut lo) = (0, 0);
    let (mut max, mut min) = (std::i32::MIN, std::i32::MAX);

    let len = nums.len();
    for idx in 0..len {
        max = if nums[idx] >= max { nums[idx] } else { hi=idx; max };
        let lo_idx = len-idx-1;
        min = if nums[lo_idx] <= min { nums[lo_idx] } else { lo = lo_idx; min };
    }
    if hi == lo { 0 } else { (hi - lo) as i32 + 1 }
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
    #[test]
    fn it_works_07() {
        assert_eq!(find_unsorted_subarray(vec![6, 5, 3, 2]), 4);
    }
    #[test]
    fn it_works_08() {
        assert_eq!(find_unsorted_subarray(vec![2, 6, 2]), 2);
    }
}
