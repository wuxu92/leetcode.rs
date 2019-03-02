// @url https://leetcode.com/problems/subarray-sum-equals-k/

// Given an array of integers and an integer k, you need to find
// the total number of continuous subarrays whose sum equals to k

// loop
#[allow(dead_code)]
pub fn subarray_sum_1(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    for low in 0..nums.len() {
        let mut sum = nums[low];
        if sum == k { count += 1; }
        for hi in low+1..nums.len() {
            sum += nums[hi];
            if sum == k { count += 1; }
        }
    }
    count
}

// use hashmap
// save sum = sum([0..idx]) => map[sum]++
//| pre=sum-k | k |
// * * * * * * * * *
#[allow(dead_code)]
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut mark : HashMap<i32, i32> = HashMap::new();
    mark.insert(0, 1);
    let (mut sum, mut count) = (0, 0);
    for num in nums {
        sum += num;
        if let Some(pre_sum) = mark.get(&(sum-k)) {
            count += pre_sum;
        }
        // insert sum to mark
        mark.entry(sum).and_modify(|v| {*v += 1}).or_insert(1);
    }
    count
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(subarray_sum(vec![1,1,1], 2), 2);
    }
}
