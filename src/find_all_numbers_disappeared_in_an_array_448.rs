// @url https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/

#[allow(dead_code)]
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
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
