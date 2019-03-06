// @url https://leetcode.com/problems/maximal-rectangle/
//

fn pp(s: &str, vv: &Vec<Vec<usize>> ) {
    println!("{}", s);
    for vec in vv {
        println!("{:?}", vec);
    }
}

#[allow(dead_code)]
pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let mm = matrix;
    if mm.len() == 0 || mm[0].len() == 0 { return 0 }
    let (n, m) = (mm.len(), mm[0].len());   // n rows, m cols
    let (mut left, mut down) = (vec![vec![0;m+1];n+1], vec![vec![0;m+1];n+1]);
    let mut max = 0;
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if mm[i][j] == '0' { left[i][j] = 0; down[i][j] = 0; }
            else {
                left[i][j] = left[i][j+1] + 1;
                down[i][j] = down[i+1][j] + 1;
            }
        }
    }
    // pp("left: ", &left);
    // pp("down: ", &down);
    for i in 0..n {
        for j in 0..m {
            if mm[i][j] == '0' { continue }
            let mut high = n;
            for idx in 0..left[i][j] {
                if down[i][j+idx] < high { high = down[i][j+idx]; }
                if (idx+1) * high > max { max = (idx+1)*high; }
            }
        }
    }
    max as i32
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![
            vec!['1','0','1','0','0'],
            vec!['1','0','1','1','1'],
            vec!['1','1','1','1','1'],
            vec!['1','0','0','1','0']
        ];
        assert_eq!(maximal_rectangle(ins), 6);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![
            vec!['1','0','1','0','0'],
            vec!['1','1','1','1','1'],
            vec!['1','1','1','1','1'],
            vec!['1','0','0','1','0']
        ];
        assert_eq!(maximal_rectangle(ins), 10);
    }
    #[test]
    fn it_works_03() {
        let ins = vec![
            vec!['1','0','1','0','0'],
            vec!['0','0','1','0','1'],
            vec!['1','1','0','1','1'],
            vec!['1','0','0','1','0']
        ];
        assert_eq!(maximal_rectangle(ins), 2);
    }
    #[test]
    fn it_works_04() {
        let ins = vec![
            vec!['1','0','1','0','0'],
        ];
        assert_eq!(maximal_rectangle(ins), 1);
    }
    #[test]
    fn it_works_05() {
        let ins = vec![
            vec!['0'],
        ];
        assert_eq!(maximal_rectangle(ins), 0);
    }
    #[test]
    fn it_works_06() {
        let ins = vec![
            vec!['1'],
        ];
        assert_eq!(maximal_rectangle(ins), 1);
    }
    #[test]
    fn it_works_07() {
        let ins = vec![
            vec!['1','1','1','1','1','1','1','1'],
            vec!['1','1','1','1','1','1','1','0'],
            vec!['1','1','1','1','1','1','1','0'],
            vec!['1','1','1','1','1','0','0','0'],
            vec!['0','1','1','1','1','0','0','0']
        ];
        assert_eq!(maximal_rectangle(ins), 21);
    }
    #[test]
    fn it_works_08() {
        let ins = vec![
            vec!['0','0','1','0'],
            vec!['1','1','1','1'],
            vec!['1','1','1','1'],
            vec!['1','1','1','0'],
            vec!['1','1','0','0'],
            vec!['1','1','1','1'],
            vec!['1','1','1','0']
        ];
        assert_eq!(maximal_rectangle(ins), 12);
    }
    #[test]
    fn it_works_09() {
        let ins = vec![
            vec!['1','0','1','1','0','1'],
            vec!['1','1','1','1','1','1'],
            vec!['0','1','1','0','1','1'],
            vec!['1','1','1','0','1','0'],
            vec!['0','1','1','1','1','1'],
            vec!['1','1','0','1','1','1']
        ];
        assert_eq!(maximal_rectangle(ins), 8);
    }
}
