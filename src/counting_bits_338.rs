// @url https://leetcode.com/problems/counting-bits/

// Loop version
#[allow(dead_code)]
pub fn count_bits_01(num: i32) -> Vec<i32> {
    let mut res : Vec<i32> = vec![0; num as usize+1];
    fn bits(x: i32) -> i32 {
        let (mut x, mut cnt) = (x, 0);
        while x > 0 {
            cnt += 1;
            x &= x-1;
        }
        cnt
    }
    for i in 0..num+1 { res.push(bits(i)); }
    res
}

// dfs
#[allow(dead_code)]
pub fn count_bits_02(num: i32) -> Vec<i32> {
    let mut res : Vec<i32> = vec![0; num as usize+1];
    fn process(res: &mut Vec<i32>, x: usize) -> i32 {
        // println!("process: {}, res: {:?}", x, res);
        if x == 0 { return 0 }
        if res[x] > 0 { return res[x] }
        res[x] = process(res, x&(x-1))+1;
        res[x]
    }
    let max = num as usize + 1;
    for i in (1..max).rev() {
        if res[i] == 0 { process(&mut res, i); }
    }
    res
}

// another one
#[allow(dead_code)]
pub fn count_bits(num: i32) -> Vec<i32> {
    let num = num as usize;
    let mut res : Vec<i32> = vec![0; num+1];
    for i in 1..num+1 {
        res[i]= res[i>>1] + (i&1) as i32;
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(count_bits(5), vec![0,1,1,2,1,2]);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(count_bits(2), vec![0,1, 1]);
    }
}
