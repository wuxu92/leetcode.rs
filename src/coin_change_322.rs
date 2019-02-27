// @url https://leetcode.com/problems/coin-change/

#[allow(dead_code)]
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut coins = coins;
    let amount = amount as usize;
    coins.sort_unstable();
    let mut dp : Vec<i32> = vec![-1; amount+2];
    dp[0] = 0;
    for i in 0..amount as usize {
        if dp[i] == -1 { continue }
        for &j in coins.iter() {
            if j <= 0 { continue }
            let dst = i + j as usize;
            if dst > amount+1 { break;}
            if dp[i] + 1 < dp[dst] || dp[dst] == -1 { dp[dst] = dp[i] + 1; }
        }
        // println!("dp: {:?}", dp);
    }
    dp[amount] as i32
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(coin_change(vec![2], 3), -1);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }
}
