// @url https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut min = std::i32::MAX;
    for p in prices {
        if p <= min { min = p; continue }
        if res < p - min { res = p - min }
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![7,1,5,3,6,4];
        assert_eq!(max_profit(ins), 5);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![7,6,4,3,1];
        assert_eq!(max_profit(ins), 0);
    }
}
