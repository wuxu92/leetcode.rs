// @url https://leetcode.com/problems/unique-paths/

pub fn unique_paths_1(m: i32, n: i32) -> i32 {
    // let mut grid : [i32; m*n] = [0;m*n];
    // m column, n row
    let mut grid : Vec<i32> = vec![0; (m*n) as usize];

    for r in 0..n { grid[(m*r) as usize] = 1;}
    for l in 0..m { grid[l as usize] = 1;}
    for r in 1..n {
        for l in 1..m {
            let idx = (r * m + l) as usize;
            grid[idx] = grid[idx-1] + grid[idx-m as usize];
        }
    }
    grid[(m*n - 1) as usize]
}

// m-1 stop to right, n-1 step to down
// so furmula as A(n-1, m+n-2) + A(m-1, m+n-2)
//  = (m-1+n-1)!/((m-1)!*(n-1)!) = (m+1*m+2*...*m+n-1)/(1*2*3...*n-1)
pub fn unique_paths(m: i32, n: i32) -> i32 {
    if m ==1 || n == 1 { return 1 }
    let mut res :i64= 1;
    for r in m..m+n-1 {
        res *= r as i64;
        res /= (r-m+1) as i64;
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(unique_paths(7, 3), 28);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(unique_paths(3, 2), 3);
    }
    #[test]
    fn it_works_3() {
        assert_eq!(unique_paths(7, 3), 28);
    }
}
