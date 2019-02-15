// @url https://leetcode.com/problems/word-search/

// recursive version
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    fn search(grid: &mut Vec<Vec<char>>, x: usize, y: usize, w: &[u8], idx: usize) -> bool {
        if w.len() <= idx { return true }
        let n = grid.len();
        let m = grid[0].len();
        let next_ch = w[idx] as char;
        let pilot = vec![vec![-1, 0], vec![1,0], vec![0, -1], vec![0, 1]];
        let old_ch = grid[x][y];    // save old status
        grid[x][y] = 0 as char;

        for vec in pilot {
            let xx = x as i32 + vec[0];
            let yy = y as i32 + vec[1];
            if xx >= 0 && xx < n as i32 && yy >=0 && yy < m as i32 {
                let ix = xx as usize;
                let iy = yy as usize;
                if grid[ix][iy] == next_ch {
                    if search(grid, ix, iy, w, idx+1) { return true }
                }
            }
        }
        grid[x][y] = old_ch;    // restore after this recursion
        false
    }
    // search start point
    let bytes = word.as_bytes();
    let first_ch = bytes[0] as char;
    let mut copy = board.clone();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == first_ch {
                if search(&mut copy, i, j, bytes, 1) { return true }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(exist(
                vec![
                   vec!['A','B','C','E'],
                   vec!['S','F','C','S'],
                   vec!['A','D','E','E']
                ], String::from("ABCCED")), true
        );
    }
    #[test]
    fn it_works_2() {
        assert_eq!(exist(
                vec![
                   vec!['A','B','C','E'],
                   vec!['S','F','C','S'],
                   vec!['A','D','E','E']
                ], String::from("SEE")
                ), true
        );
    }
    #[test]
    fn it_works_3() {
        assert_eq!(exist(
                vec![
                   vec!['A','B','C','E'],
                   vec!['S','F','C','S'],
                   vec!['A','D','E','E']
                ], String::from("ABCB")), false
        );
    }
    #[test]
    fn it_works_4() {
        assert_eq!(exist(
                vec![
                   vec!['a','a'],
                ], String::from("aaa")), false
        );
    }
}
