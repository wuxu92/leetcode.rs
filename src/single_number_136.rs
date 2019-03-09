// @url https://leetcode.com/problems/single-number/

#[allow(dead_code)]
pub fn single_number_1(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 0 { return  0 }
    let (mut idx, mut res) = (1, nums[0]);
    while idx < len {
        res ^= nums[idx];
        idx += 1;
    }
    res
}

// use fold
#[allow(dead_code)]
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |res, x| res ^ x)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![2,2,1];
        assert_eq!(single_number(ins), 1);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![4,1,2,1,2];
        assert_eq!(single_number(ins), 4);
    }
    #[test]
    fn it_works_03() {
        let ins = vec![4];
        assert_eq!(single_number(ins), 4);
    }
    #[test]
    fn it_works_04() {
        let ins = vec![];
        assert_eq!(single_number(ins), 0);
    }
}
