// @url https://leetcode.com/problems/sudoku-solver/

struct Gter {
    num: usize,
    idx: u8
}

impl Gter {
    #[inline]
    pub fn new(num: usize) -> Self {
        Gter{ num: num, idx: 1 }
    }
}

impl Iterator for Gter {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        while self.num > 0 {
            if self.num & 1 == 0 { self.idx += 1; self.num >>= 1; }
            else {
                self.num >>= 1;
                return Some((self.idx + '0' as u8) as char)
            }
        }
        None
    }
}

fn count_1bit(num: usize) -> usize {
    let (mut count, mut num) = (0, num);
    while num != 0 {
        num &= num-1;
        count += 1;
    }
    count
}

// Wronged
#[allow(dead_code)]
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    // build vec list
    let mut wait : Vec<usize> = vec![0b111111111; 81]; // bit set
    let mut todo : Vec<usize> = vec![];   // 待处理列表

    let num_0 = '1' as u8;
    for i in 0..9 {
        for j in 0..9 {
            let idx = i*9+j;
            if board[i][j] != '.' {
                let num = board[i][j] as u8 - num_0;
                let bitset = !(1<<num); // 0b11101111
                let (x_idx, y_idx) = (i-i%3, j-j%3);
                for k in 0..9 {
                    wait[i*9+k] &= bitset;  // 对应位设0
                    wait[k*9+j] &= bitset;
                    let (nx, ny) = (x_idx+k/3, y_idx+k%3);
                    wait[nx*9+ny] &= bitset;
                }
                wait[idx] = 0;
            } else {
                todo.push(idx as usize);
            }
        }
    }
    for i in 0..9 {
        println!("get wait: {:?}", &wait[i*9..i*9+9]);
    }
    println!("get todo: {:?}", todo);
    // process
    fn process(board: &mut Vec<Vec<char>>, wait: Vec<usize>, todo: &mut Vec<usize>) -> bool {
        if todo.len() == 0 {return true}
        // 选择待处理数最少的一个
        let (mut loc, mut min) = (0, std::usize::MAX);
        for idx in 0..todo.len() {
            if wait[todo[idx]] == 0 { continue }
            let count1=  count_1bit(wait[todo[idx]]);
            if count1 < min { min = count1; loc = idx; }
        }
        // 遍历这个待处理节点
        let wloc = todo[loc];
        let wait_num = wait[wloc];
        let g = Gter::new(wait_num);
        // 设置这个todo已经结束
        todo.remove(loc);
        let (i, j) = (wloc/9, wloc%9);
        println!("process: ({},{}), wait_num: {}", i, j, wait_num);
        for ch in g {
            let mut nwait = wait.clone();
            nwait[wloc] = 0;
            let bitset = 1<<ch as u8 - '1' as u8; // 0b00010000
            // 检查是否符合要求
            let (x_idx, y_idx) = (i-i%3, j-j%3);
            println!("process: {} {} {} {}", i, j, ch, wloc);
            let mut has_err = false;
            for k in 0..9 {
                let (nx, ny) = (x_idx+k/3, y_idx+k%3);
                if board[i][k] == ch || board[k][j] == ch || board[nx][ny] == ch {
                    println!("err as k={} nxy: ({},{}), {} {} {}", k, nx, ny, board[i][k], board[k][j], board[nx][ny]);
                    print_sudoku(board);
                    has_err = true;
                    break
                }
                nwait[i*9+k] &= !bitset;
                nwait[k*9+j] &= !bitset;
                nwait[nx*9+ny] &= !bitset;
                nwait[i*9+j] = 0;
            }
            if has_err { continue; }
            // 处理下一个
            board[i][j] = ch;
            println!("set boared[{}][{}] as {}", i, j, ch);
            if process(board, nwait, todo) { return true }
            // restore
        }
        println!("restore: ({},{}) as wloc:{}", i, j, wloc);
        board[i][j] = '.';
        // wait[wloc] = wait_num;
        todo.push(wloc);
        false
    }

    process(board, wait, &mut todo);
}

#[allow(dead_code)]
fn print_sudoku(board: &Vec<Vec<char>>) {
    for vec in board {
        println!("{:?}", vec);
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // #[test]
    // fn test_count1() {
    //     assert_eq!(count_1bit(0b001100011), 4);
    //     assert_eq!(count_1bit(0b0000011), 2);
    //     assert_eq!(count_1bit(0b000001), 1);
    //     assert_eq!(count_1bit(0b0), 0);
    // }

    #[test]
    fn it_works_01() {
        let mut input = vec![
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
        print_sudoku(&input);
        solve_sudoku(&mut input);
        print_sudoku(&input);
    }
    #[test]
    fn it_works_02() {
        let mut input = vec![
            vec!['.','.','9','7','4','8','.','.','.'],
            vec!['7','.','.','.','.','.','.','.','.'],
            vec!['.','2','.','1','.','9','.','.','.'],
            vec!['.','.','7','.','.','.','2','4','.'],
            vec!['.','6','4','.','1','.','5','9','.'],
            vec!['.','9','8','.','.','.','3','.','.'],
            vec!['.','.','.','8','.','3','.','2','.'],
            vec!['.','.','.','.','.','.','.','.','6'],
            vec!['.','.','.','2','7','5','9','.','.']
        ];
        print_sudoku(&input);
        solve_sudoku(&mut input);
        print_sudoku(&input);
    }
}
