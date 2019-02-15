// @url https://leetcode.com/problems/minimum-path-sum/

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len()==0 || grid[0].len() == 0 { return 0 }
    let n = grid.len();
    let m = grid[0].len();
    let mut scores : Vec<i32> = vec![0; n*m];
    // init borders
    scores[0] = grid[0][0];
    for i in 1..m {
        scores[i] = scores[i-1] + grid[0][i];
    }
    for i in 1..n {
        scores[i*m] = scores[i*m-m] + grid[i][0];
    }

    for i in 1..n {
        for j in 1..m {
            let up = scores[i*m+j-m];
            let left = scores[i*m+j-1];
            scores[i*m+j] = grid[i][j] + if up < left {up} else {left};
        }
    }
    scores[n*m-1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(min_path_sum(
                vec![
                vec![1,3,1],
                vec![1,5,1],
                vec![4,2,1]
                ]) , 7);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(min_path_sum(
                vec![
                vec![1,3,1],
                vec![1,5,1],
                ]) , 6);
    }
    #[test]
    fn it_works_3() {
        assert_eq!(min_path_sum(
                vec![
                vec![1,3,1],
                ]) , 5);
    }
    #[test]
    fn it_works_4() {
        assert_eq!(min_path_sum(
                vec![
                vec![1],
                ]) , 1);
    }
    #[test]
    fn it_works_5() {
        assert_eq!(min_path_sum(
                vec![
                vec![0],
                ]) , 0);
    }
    #[test]
    fn it_works_6() {
        assert_eq!(min_path_sum(
                vec![
                vec![0],
                vec![3],
                vec![6],
                ]) , 9);
    }
    #[test]
    fn it_works_7() {
        assert_eq!(min_path_sum(
                vec![
                ]) , 0);
    }
}
