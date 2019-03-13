// @url https://leetcode.com/problems/regular-expression-matching/

// Wronged
#[allow(dead_code)]
pub fn is_match_err_1(s: String, p: String) -> bool {
    let bs : Vec<_> = s.chars().collect();
    let ps : Vec<_> = p.chars().collect();

    let (mut pi, mut bi) = (0, 0);
    while pi < ps.len() {
        if bi == bs.len() { return false }
        match ps[pi] {
            '.' => {
                pi += 1;
                if pi < ps.len() && ps[pi] == '*' {
                    if pi == ps.len()-1 { return true }
                    let nch = ps[pi+1];
                    while bi < bs.len() {
                        if bs[bi] == nch { break; }
                        bi += 1;
                    }
                } else {
                    if bi == s.len() { return false; }
                    bi += 1;
                }
            },
            _ => {
                if pi < ps.len()-1 && ps[pi+1] == '*' {
                    pi += 1;
                    let pre = ps[pi-1];
                    // if after same as pre
                    pi += 1; // 
                    while pi < ps.len() && ps[pi] == pre { pi += 1; }
                    while bi < bs.len() {
                        if bs[bi] != pre { break }
                        bi += 1;
                    }
                } else {
                    if ps[pi] != bs[bi] { return false; }
                    pi += 1; bi += 1;
                }
            },
        }
    }
    bi == s.len()
}

// DP
#[allow(dead_code)]
pub fn is_match(s: String, p: String) -> bool {
    let (lens, lenp) = (s.len(), p.len());
    if lens == 0 && lenp == 0 { return true }
    let (bs, bp) = (s.chars().collect::<Vec<char>>(), p.chars().collect::<Vec<char>>());
    let mut dp = vec![vec![false; lenp+1]; lens+1];
    dp[0][0] = true;
    for i in 1..lenp {
        if bp[i] == '*' && dp[0][i-1] {
            dp[0][i+1] = true;
        }
    }
    for i in 0..lens { // bs
        for j in 0..lenp { // bp
            let (ii, jj) = (i+1, j+1);
            if bs[i] == bp[j] || bp[j] == '.' { dp[ii][jj] = dp[i][j]; }
            else if bp[j] == '*' {
                if bp[j-1] != bs[i] && bp[j-1] != '.' {
                    dp[ii][jj] = dp[ii][j];
                } else {
                    dp[ii][jj] = dp[i][j] || dp[ii][j] || dp[ii][j-1];
                }
            }
        }
    }
    dp[lens][lenp]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let s = String::from("aa");
        let p = String::from("a");
        assert_eq!(is_match(s, p), false);
    }
    #[test]
    fn it_works_02() {
        let s = String::from("aa");
        let p = String::from("a*");
        assert_eq!(is_match(s, p), true);
    }
    #[test]
    fn it_works_03() {
        let s = String::from("aabbbbbb");
        let p = String::from("a*b*");
        assert_eq!(is_match(s, p), true);
    }
    #[test]
    fn it_works_04() {
        let s = String::from("aabbbbbb");
        let p = String::from("a*b*c");
        assert_eq!(is_match(s, p), false);
    }
    #[test]
    fn it_works_05() {
        let s = String::from("aab");
        let p = String::from("c*a*b");
        assert_eq!(is_match(s, p), true);
    }
    #[test]
    fn it_works_06() {
        let s = String::from("mississippi");
        let p = String::from("mis*is*p*.");
        assert_eq!(is_match(s, p), false);
    }
    #[test]
    fn it_works_07() {
        let s = String::from("aaa");
        let p = String::from("ab*a*c*a");
        assert_eq!(is_match(s, p), false);
    }
}
