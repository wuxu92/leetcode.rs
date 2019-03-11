// @url https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/

// dp for status change
// buy[i] = max(buy[i-1], sell[i-2] - p[i])
// sell[i] = max(sell[i]-1, buy[i-1] + p[i])
#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    // max profit in i-th day by buy, sell, or rest
    let len = prices.len();
    if len <= 1 { return 0 }
    let (mut buy, mut sell) = (vec![0;len], vec![0;len]);
    buy[0] = -1 * prices[0];
    let delt = prices[1] - prices[0];
    if delt > 0 { sell[1] = delt; buy[1] = buy[0]; } else { buy[1] = -1 * prices[1]; };

    let max = |a, b| {
        if a > b { a } else { b }
    };
    for i in 2..len {
        buy[i] = max(buy[i-1], sell[i-2] - prices[i]);
        sell[i] = max(sell[i-1], buy[i-1] + prices[i]);
    }
    sell[len-1]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(max_profit(vec![1,2,3,0,2]), 3);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(max_profit(vec![1,2]), 1);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(max_profit(vec![1,2,3]), 2);
    }
    #[test]
    fn it_works_04() {
        assert_eq!(max_profit(vec![1]), 0);
    }
}
