
// @url https://leetcode.com/problems/maximum-length-of-repeated-subarray/
#[allow(dead_code)]
pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
    if a.len() ==0 || b.len() == 0 { return 0 }
    let (mut res, mut mark) = (0, vec![0; a.len()]);

    for i in 0..a.len() {
        let mut pre = mark[0];
        mark[0] = if a[i] == b[0] { 1 } else {0};
        if mark[0] > res { res = mark[0]; }
        for j in 1..b.len() {
            let v_j = mark[j];
            mark[j] = if a[i] ==b[j] {pre + 1} else {0};
            if mark[j] > res { res = mark[j]; }
            pre = v_j;
        }
        // println!("mark: {:?}", mark);
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let v1 = vec![1,2,3,2,1];
        let v2 = vec![3,2,1,4,7];
        assert_eq!(find_length(v1, v2), 3);
    }
}
