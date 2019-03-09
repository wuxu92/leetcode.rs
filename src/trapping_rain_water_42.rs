// @url https://leetcode.com/problems/trapping-rain-water/

// Wrong
#[allow(dead_code)]
pub fn trap_err_1(height: Vec<i32>) -> i32 {
    // find the fisrt top
    let (mut res, h) = (0, height);
    let len = h.len();
    let (mut lmax, mut rmax) = (vec![0;len], vec![0; len]);
    let (mut l, mut r) = (0, len-1);
    for idx in 0..len {
        if h[idx] >= h[l] { l= idx; }
        lmax[idx] = l;
        let r_idx = len-1-idx;
        if h[r_idx] >= h[r] { r = r_idx; }
        rmax[r_idx] = r;
    }
    let mut idx = 0;
    while idx < len {
        let (left, right) = (lmax[idx], rmax[idx]);
        if left >= right - 1 { idx += 1; continue; }
        let max = if h[left] > h[right] { h[right] } else { h[left] };
        for jdx in left+1..right+1 {
            println!("idx: {}, jdx: {}, {} hl: {}", idx, jdx, h[jdx], h[left]);
            if max >= h[jdx] { res += max - h[jdx]; idx = jdx; }
            else if jdx > idx { idx = jdx; break; }
        }
    }
    res
}

pub fn trap(height: Vec<i32>) -> i32 {
    if height.len() == 0 { return 0 }
    let h = height;
    let (mut l, mut r) = (0, h.len()-1);
    let (mut lmax, mut rmax) = (0, 0);
    let mut res = 0;
    while l < r {
        if h[l] < h[r] {
            if h[l] >= lmax { lmax = h[l]; }
            else { res += lmax - h[l]; }
            l += 1;
        } else {
            if h[r] >= rmax { rmax = h[r]; }
            else { res += rmax- h[r]; }
            r -= 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        assert_eq!(trap(ins), 6);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![];
        assert_eq!(trap(ins), 0);
    }
}
