// @url https://leetcode.com/problems/find-the-duplicate-number/

// find a circle in a linkedlist
// fast/slow stop forward solution
#[allow(dead_code)]
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 { return 0 }
    let (mut slow, mut fast) = (nums[0] as usize, nums[nums[0] as usize] as usize);
    while fast != slow {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
    }
    // find entry
    fast = 0;
    while fast != slow {
        slow = nums[slow] as usize;
        fast = nums[fast] as usize;
    }
    slow as i32
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(find_duplicate(vec![1,3,4,2,2]), 2);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(find_duplicate(vec![3,1,3,4,2]), 3);
    }
}
