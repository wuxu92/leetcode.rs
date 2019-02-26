// @url https://leetcode.com/problems/perfect-squares/

// dp solution
#[allow(dead_code)]
pub fn num_squares(n: i32) -> i32 {
    // use btreemap instead of vec
    let n = n as usize;
    let mut v : Vec<i32> = vec![0; n+4];
    v[1] = 1; v[2] = 2; v[3] = 3; v[4]=1;
    for i in 5..n+1 {
        let mut j = 1;
        let mut min = std::i32::MAX;
        while j*j < i {
            if v[i-j*j] < min { min = v[i-j*j]; }
            j+=1;
        }
        if j*j == i { min = 0; }
        v[i] = min + 1;
    }
    v[n]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(num_squares(12), 3);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(num_squares(5), 2);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(num_squares(3), 3);
    }
    #[test]
    fn it_works_04() {
        assert_eq!(num_squares(10000), 1);
    }
}

