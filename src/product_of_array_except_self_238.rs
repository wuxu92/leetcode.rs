// @url https://leetcode.com/problems/product-of-array-except-self/

#[allow(dead_code)]
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    if n == 0 { return vec![] }
    let mut res : Vec<i32> = vec![0;n];
    let mut part_res = vec![1; n];
    for i in 1..nums.len() {
        part_res[i] = part_res[i-1] * nums[i-1];
    }
    let mut r = 1;
    for i in (0..nums.len()).rev() {
        res[i] = part_res[i] * r;
        r *= nums[i];
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(product_except_self(vec![1, 0]), vec![0, 1]);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(product_except_self(vec![1,2]), vec![2, 1]);
    }
    #[test]
    fn it_works_04() {
        assert_eq!(product_except_self(vec![1,2,0,3]), vec![0, 0, 6, 0]);
    }
}

