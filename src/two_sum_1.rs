// @url https://leetcode.com/problems/two-sum/

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut mark : HashMap<i32, usize>  = HashMap::new();
    for idx in 0..nums.len() {
        if let Some(pos) = mark.get(&(target-nums[idx])) {
            return vec![*pos as i32,idx as i32]
        } 
        mark.insert(nums[idx], idx);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0,1]);
    }
}
