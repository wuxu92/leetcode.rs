// @url https://leetcode.com/problems/integer-to-roman/

#[allow(dead_code)]
pub fn int_to_roman(num: i32) -> String {
    let mut res = String::new();
    // I             1
    // V             5
    // X             10
    // L             50
    // C             100
    // D             500
    // M             1000

    res
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
