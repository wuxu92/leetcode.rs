// @url https://leetcode.com/problems/target-sum/

#[allow(dead_code)]
pub fn find_target_sum_ways_1(nums: Vec<i32>, s: i32) -> i32 {
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

// subset P/N solution
// sum(P) - sum(N) = s
// sum(P) - sum(N) + sun(P) + sum(N) = s + sum(P) + sum(N) = s + sum(nums)
// 2 * sum(p) = s + sum(nums)
#[allow(dead_code)]
pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
    let sum : i32 = nums.iter().sum();
    if sum < s || (sum + s) & 0b1 == 1 { return 0 }
    let sum = (sum + s) as usize / 2;
    // subset sum eq sum
    let mut dp : Vec<i32> = vec![0; sum + 1];
    dp[0] = 1;  // as give one more 0
    for &num in nums.iter() {
        let num = num as usize;
        let mut idx = sum;
        while idx >= num {
            dp[idx] += dp[idx-num];
            if idx == 0 { break }
            idx -= 1;
        }
    }
    dp[sum]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(find_target_sum_ways(vec![1000], -1000), 1);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(find_target_sum_ways(vec![0, 1], 1), 2);
    }
}

