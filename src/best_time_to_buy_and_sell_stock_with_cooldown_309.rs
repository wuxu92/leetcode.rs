// @url https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/

// dp for status change
#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    // max profit in i-th day by buy, sell, or rest
    let (mut buy, mut sell, mut rest) = (0, 0, 0);
    0
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(2 + 2, 4);
    }
}
