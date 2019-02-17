// @url https://leetcode.com/problems/valid-sudoku/

#[allow(dead_code)]
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    if board.len() != 9 || board[0].len() != 9 { return false }
    let check = |s: &[char] | -> bool {
        for idx in 0..s.len() {
            let ch = s[idx];
            if ch == '.' { continue }
            if ch < '1' || ch > '9' || s[0..idx].contains(&ch) { return false }
        }
        true
    };
    for i in 0..9 {
        if ! check(&board[i][0..9]) { return false; }
        let mut vec : Vec<char> = vec![];
        for j in 0..9 { vec.push(board[j][i]); }
        if ! check(&vec[..]) { return false};
    }
    for i in 0..3 {
        for j in 0..3 {
            let mut vec : Vec<char> = vec![];
            let (idx, jdx) = (i*3, j*3);
            for k in 0..3 {
                for l in 0..3 { vec.push(board[idx+k][jdx+l]); }
            }
            if ! check(&vec[..]) { return false};
        }
    }
    true
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let input = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        assert_eq!(is_valid_sudoku(input), true);
    }
    #[test]
    fn it_works_02() {
        let input = vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        assert_eq!(is_valid_sudoku(input), false);
    }
    #[test]
    fn it_works_03() {
        let input = vec![
            vec!['.','.','4','.','.','.','6','3','.'],vec!['.','.','.','.','.','.','.','.','.'],vec!['5','.','.','.','.','.','.','9','.'],vec!['.','.','.','5','6','.','.','.','.'],vec!['4','.','3','.','.','.','.','.','1'],vec!['.','.','.','7','.','.','.','.','.'],vec!['.','.','.','5','.','.','.','.','.'],vec!['.','.','.','.','.','.','.','.','.'],vec!['.','.','.','.','.','.','.','.','.']
        ];
        assert_eq!(is_valid_sudoku(input), false);
    }
}
