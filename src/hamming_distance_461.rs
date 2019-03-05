
// @url https://leetcode.com/problems/hamming-distance/

#[allow(dead_code)]
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(hamming_distance(1, 4), 2);
    }
}
