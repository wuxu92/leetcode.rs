// @url https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/

#[allow(dead_code)]
pub fn find_disappeared_numbers_1(nums: Vec<i32>) -> Vec<i32> {
    let mut mark: Vec<bool> = vec![true; nums.len()+1];
    let mut res = vec![];
    for num in nums {
        mark[num as usize] = false;
    }
    for idx in 1..mark.len() {
        if mark[idx] { res.push(idx as i32);}
    }
    res
}

#[allow(dead_code)]
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let (mut nums, mut res) = (nums, vec![]);
    for idx in 0..nums.len() {
        let abs = if nums[idx] >= 0 { nums[idx] } else { -1 * nums[idx]};
        let abs = abs as usize - 1;
        nums[abs] = if nums[abs] < 0 { nums[abs] } else { -1 * nums[abs] }
    }
    for idx in 0..nums.len() {
        if nums[idx] > 0 { res.push(idx as i32 + 1); }
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![4,3,2,7,8,2,3,1];
        let out = vec![5,6];
        assert_eq!(find_disappeared_numbers(ins), out);
    }
}
