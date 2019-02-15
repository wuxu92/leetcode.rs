// @url https://leetcode.com/problems/climbing-stairs/

pub fn climb_stairs(n: i32) -> i32 {
    let mut count : Vec<i32> = vec![1; n as usize +3];
    for idx in 2..n+1 {
        let i = idx as usize;
        count[i] = count[i-1] + count[i-2];
    }
    count[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(climb_stairs(2), 2);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(climb_stairs(3), 3);
    }
}
