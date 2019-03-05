// @url https://leetcode.com/problems/queue-reconstruction-by-height/

#[allow(dead_code)]
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res : Vec<Vec<i32>> = vec![];
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let input = vec![
            vec![7,0], 
            vec![4,4], 
            vec![7,1], 
            vec![5,0], 
            vec![6,1], 
            vec![5,2]
        ];
        let out = vec![vec![5,0], vec![7,0], vec![5,2], vec![6,1], vec![4,4], vec![7,1]];
        assert_eq!(reconstruct_queue(input), out);
    }
}
