// @url https://leetcode.com/problems/first-missing-positive/
// Your algorithm should run in O(n) time and uses constant extra space.

#[allow(dead_code)]
pub fn first_missing_positive_1(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len() as i32;
    let mut pre;
    for i in 0..len as usize {
        pre = nums[i];
        while pre > 0 && pre <= len && nums[pre as usize-1] != pre {
            let n = nums[pre as usize-1];
            nums[pre as usize-1] = pre;
            pre = n;
        }
    }

    println!("{:?}", nums);

    for i in 0..len {
        if nums[i as usize] != i+1 { return i+1 }
    }
    len+1
}

// use swap
#[allow(dead_code)]
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let (mut i, mut r, len) = (0, nums.len(), nums.len() as i32);

    while i < r {
        if nums[i] == i as i32 + 1 { i+=1; }
        else if nums[i] <= 0 || nums[i] > len || nums[i] == nums[nums[i] as usize-1]{
            r -= 1; nums.swap(i, r);
        } else {
            let idx = nums[i] as usize -1;
            nums.swap(idx, i);
        }
    }
    r as i32+1
}



#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![1,2,0];
        assert_eq!(first_missing_positive(ins), 3);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![3,4,-1,1];
        assert_eq!(first_missing_positive(ins), 2);
    }
    #[test]
    fn it_works_03() {
        let ins = vec![7,8,9,11,12];
        assert_eq!(first_missing_positive(ins), 1);
    }
    #[test]
    fn it_works_04() {
        let ins = vec![7];
        assert_eq!(first_missing_positive(ins), 1);
    }
    #[test]
    fn it_works_05() {
        let ins = vec![];
        assert_eq!(first_missing_positive(ins), 1);
    }
    #[test]
    fn it_works_06() {
        let ins = vec![2, 1];
        assert_eq!(first_missing_positive(ins), 3);
    }
    #[test]
    fn it_works_07() {
        let ins = vec![2, 2];
        assert_eq!(first_missing_positive(ins), 1);
    }
}
