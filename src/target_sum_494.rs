// @url https://leetcode.com/problems/target-sum/

#[allow(dead_code)]
pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
    let (mut res, mut count) = (vec![], 0);
    for num in nums {
        if res.len() == 0 { res.push(num); res.push(-1 * num); continue; }
        for idx in 0..res.len() {
            let cur = res[idx];
            res.push(cur + num);
            res[idx] -= num;
        }
    }
    for sum in res { if sum == s { count += 1; } }
    count
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }
}

