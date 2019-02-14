
// @url: https://leetcode.com/problems/longest-valid-parentheses/
// Better solution: save char index to stack instead of char itself

// Wrong
pub fn longest_valid_parentheses_1(s: String) -> i32 {
    // if s.len() == 0 { return 0 }
    let mut score = vec![0; s.len()+1];
    let mut stack = String::new();

    let mut idx = 1;
    let chars : Vec<char> = s.chars().collect();
    for ch in chars {
        // println!("ch: {}, last: {:?}", ch, stack.chars().last());
        if ch == ')' && stack.chars().last() == Some('(') {
            score[idx] = score[idx-1]+2;
            stack.pop();
        } else {
            stack.push(ch);
            score[idx] = score[idx-1];
        }
        idx += 1;
    }
    // println!("score: {:?}.", score);
    score[idx-1]
}

// Wrong: full array
pub fn longest_valid_parentheses_2(s: String) -> i32 {
    if s.len() == 0 { return 0 }
    let n = s.len();
    let mut score = vec![0; n*n+1];

    let mut row: usize;
    let mut col: usize = 1;
    // fill boundry
    let chars : Vec<char> = s.chars().collect();
    while col < n { // col start from 1
        if chars[col-1] == '(' && chars[col] == ')' {
            let idx = n * (col-1) + col;
            score[idx] = 2;
        }
        col += 1;
    }
    println!("score: {:?}.", score);
    col = 2;
    while col < n {
        row = 0;
        let tmp = col;
        while row < n - col {
            let idx = n * row + col;
            let down_idx = idx + n;
            let mut delta = 0;
            if score[down_idx-1] < score[down_idx] {
                delta = 2;
            }
            score[idx] = score[idx-1] + delta;
            row += 1;
            col += 1;
            println!("row: {}, col{}, score: {:?}.", row, col, score);
        }
        col = tmp+1;
    }
    score[n-1]
}

//DP
pub fn longest_valid_parentheses(s: String) -> i32 {
    // use two stack
    use std::collections::vec_deque::VecDeque;
    let chars : Vec<char> = s.chars().collect();
    let mut ch_stack = VecDeque::new();
    let mut scores = VecDeque::new();
    ch_stack.push_back(')');
    scores.push_back(0);

    for ch in chars {
        let top_ch = ch_stack.back().unwrap().clone();
        if top_ch == '(' && ch == ')' {
            let top_score = scores.pop_back().unwrap().clone();
            let top2_score = scores.pop_back().unwrap().clone();
            scores.push_back( top_score + 2 + top2_score);
            ch_stack.pop_back();
            continue;
        }
        scores.push_back(0);
        ch_stack.push_back(ch);
    }
    // loop to find the max score
    let mut score = -1;
    for s in  scores {
        if s > score { score = s; }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_0() {
        assert_eq!(longest_valid_parentheses(String::from("(()")), 2);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(longest_valid_parentheses(String::from("")), 0);
    }
    #[test]
    fn it_works_3() {
        assert_eq!(longest_valid_parentheses(String::from(")((((((((")), 0);
    }
    #[test]
    fn it_works_4() {
        assert_eq!(longest_valid_parentheses(String::from(")((())(()(((")), 4);
    }
    #[test]
    fn it_works_5() {
        assert_eq!(longest_valid_parentheses(String::from("(()))(()())((()")), 6);
    }
    #[test]
    fn it_works_6() {
        assert_eq!(longest_valid_parentheses(String::from("(")), 0);
    }
    #[test]
    fn it_works_7() {
        assert_eq!(longest_valid_parentheses(String::from("(((((((((()))))))))")), 18);
    }
    #[test]
    fn it_works_8() {
        assert_eq!(longest_valid_parentheses(String::from("()((()))")), 8);
    }
    #[test]
    fn it_works_9() {
        assert_eq!(longest_valid_parentheses(String::from("()(())")), 6);
    }
    #[test]
    fn it_works_10() {
        assert_eq!(longest_valid_parentheses(String::from("()(()())()()()")), 14);
    }
    #[test]
    fn it_works_11() {
        assert_eq!(longest_valid_parentheses(String::from("()(()()))()()()")), 8);
    }
}
