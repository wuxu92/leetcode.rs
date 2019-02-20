// @url https://leetcode.com/problems/maximum-product-subarray/

#[allow(dead_code)]
pub fn max_product(nums: Vec<i32>) -> i32 {

    // max of no-zore slice
    fn sum (x: &[i32]) -> i32 {
        if x.len() == 0 { return std::i32::MIN }
        let mut res = 1;
        for i in x { res *= i }
        res
    };

    fn process(nums: &[i32]) -> i32 {
        if nums.len() == 0 { return std::i32::MIN }
        let mut prod = 1;
        // let mut minus : Vec<usize> = vec![];    // index of -1
        let (mut n1, mut n2) = (-1, -1);
        for idx in 0..nums.len() {
            prod *= nums[idx];
            if nums[idx] < 0 { 
                if n1 == -1 { n1 = idx as i32;}
                n2 = idx as i32;
            }
        }
        println!("prod: {}", prod);
        if prod >= 0 { return prod }
        // need remove on negative nums
        // always has idx in minus
        let (n1, n2) = (n1 as usize, n2 as usize);
        let piles = if n1 == n2 { vec![n1] } else {vec![n1, n2]};
        // println!("nums: {:?}, piles:{:?}.", nums, piles);
        for idx in piles {
            if prod < nums[idx] { prod = nums[idx]; }
            let pre_sum = sum(&nums[0..idx]);
            if prod < pre_sum { prod = pre_sum; }
            let pre_sum = sum(&nums[idx+1..nums.len()]);
            if prod < pre_sum { prod = pre_sum; }
        }
        prod
    }
    let mut s = 0;
    let mut  has_0 = false;
    let slice = nums.as_slice();
    let mut res = std::i32::MIN;
    for idx in 0..slice.len() {
        if res < slice[idx] { res = slice[idx]; }
        if slice[idx] == 0 {
            if res < 0 { res = 0;}
            let part_sum = process(&slice[s..idx]);
            if res < part_sum { res = part_sum; }
            s = idx+1;
            has_0 = true;
        }
    }
    if ! has_0 {
        let full_sum = process(slice);
        if res < full_sum { res = full_sum;}
    }

    if s < slice.len() {
        let full_sum = process(&slice[s..]);
        if res < full_sum { res = full_sum;}
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(max_product(vec![-1, 0, -2]), 0);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(max_product(vec![2,3,-2,4]), 6);
    }
    #[test]
    fn it_works_04() {
        assert_eq!(max_product(vec![0]), 0);
    }
    #[test]
    fn it_works_05() {
        assert_eq!(max_product(vec![0, 1, 1, 1, 1]), 1);
    }
    #[test]
    fn it_works_06() {
        assert_eq!(max_product(vec![0, -1, 1, -1, 1, -1, 2, -1, 2]), 4);
    }
    #[test]
    fn it_works_07() {
        assert_eq!(max_product(vec![0, -2, -3]), 6);
    }
}
