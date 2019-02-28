// @url https://leetcode.com/problems/house-robber/

#[allow(dead_code)]
pub fn rob(nums: Vec<i32>) -> i32 {
    let (mut yes, mut no) = (0, 0);
    for num in nums {
        let old = yes;
        yes = no + num;
        no = if old > no { old } else { no };
    }
    if yes > no { yes } else { no }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(rob(vec![1,2,3,1]), 4);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(rob(vec![2,7,9,3,1]), 12);
    }
}
