// @url https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/

#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut res, mut buy) = (0, -1);
    // always buy and sell
    let len = prices.len();
    if len == 0 { return 0 }
    for i in 0..len-1 {
        if prices[i] < prices[i+1] && buy == -1 { buy = prices[i]; }
        if prices[i] >= prices[i+1] && buy >= 0 { res += prices[i] - buy; buy = -1; }
    }
    if buy >= 0 && len > 0 {
        let last = prices[len-1];
        if last > buy { res += last-buy; buy = 0; }
    }
    if buy > 0 { res += buy; }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![7,1,5,3,6,4];
        assert_eq!(max_profit(ins), 7);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![7,6,4,3,1];
        assert_eq!(max_profit(ins), 0);
    }
    #[test]
    fn it_works_03() {
        let ins = vec![1,2,3,4,5];
        assert_eq!(max_profit(ins), 4);
    }
    #[test]
    fn it_works_04() {
        let ins = vec![];
        assert_eq!(max_profit(ins), 0);
    }
    #[test]
    fn it_works_05() {
        let ins = vec![2,1,2,0,1];
        assert_eq!(max_profit(ins), 2);
    }
}
