// @url https://leetcode.com/problems/maximal-square/

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.len() == 0 || matrix[0].len() == 0 { return 0 }
    let mut max = 0;
    let (n, m) = (matrix.len(), matrix[0].len());
    let mut mark : Vec<Vec<usize>> = vec![vec![0;m]; n];
    for i in 0..n {
        if matrix[i][0] == '1' { mark[i][0] = 1; max = 1;}
    }
    for i in 0..m {
        if matrix[0][i] == '1' { mark[0][i] = 1; max = 1;}
    }
    let mm = matrix;
    for i in 1..n {
        for j in 1..m {
            if mm[i][j] == '0' { continue }
            mark[i][j] = 1;
            let lsize = mark[i-1][j-1];
            let mut size = 1;
            for k in 1..lsize+1 {
                if mm[i-k][j] == '0' || mm[i][j-k] == '0' { break }
                size += 1;
            }
            mark[i][j] = size;
            if mark[i][j]+ 1 > max { max = mark[i][j]; }
        }
    }
    (max * max) as i32
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let v = vec![
            vec!['1','0','1','0','0'],
            vec!['1','0','1','1','1'],
            vec!['1','1','1','1','1'],
            vec!['1','0','0','1','0']
        ];
        assert_eq!(maximal_square(v), 4);
    }
    #[test]
    fn it_works_02() {
        let v = vec![
            vec!['1','0','1','0','0'],
            vec!['1','0','1','1','1'],
            vec!['1','1','1','1','1'],
            vec!['1','0','1','1','1']
        ];
        assert_eq!(maximal_square(v), 9);
    }
    #[test]
    fn it_works_03() {
        let v = vec![
            vec!['1','0','1','0','0'],
            vec!['1','0','1','1','1'],
        ];
        assert_eq!(maximal_square(v), 1);
    }
    #[test]
    fn it_works_04() {
        let v = vec![
            vec!['0'],
        ];
        assert_eq!(maximal_square(v), 0);
    }
    #[test]
    fn it_works_05() {
        let v = vec![
            vec!['0','0','0','1'],
            vec!['1','1','0','1'],
            vec!['1','1','1','1'],
            vec!['0','1','1','1'],
            vec!['0','1','1','1']
        ];
        assert_eq!(maximal_square(v), 9);
    }
}
