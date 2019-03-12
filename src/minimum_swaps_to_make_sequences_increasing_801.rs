
// @url https://leetcode.com/problems/minimum-swaps-to-make-sequences-increasing/

// greedy-like
// Error
#[allow(dead_code)]
pub fn min_swap_err_01(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let (len, mut res) = (a.len(), 0);
    let (mut a, mut b) = (a, b);
    let mut can_last = true;
    for i in 1..len {
        if a[i] > a[i-1] && b[i] > b[i-1] {
            can_last = a[i] > b[i-1] && b[i] > a[i-1];
            continue
        }
        res += 1;
        // swap i in cannot swap i-1
        if ! can_last || i == len-1 {
            let t = a[i]; a[i] = b[i]; b[i] = t;
            can_last = false;
            continue
        }
        can_last = false;
        // try swap i-1
        if (a[i] - b[i]) * (a[i+1] - b[i+1]) > 0 {
            if a[i] > b[i-1] && b[i] > a[i-1] {
                let t = a[i-1]; a[i-1] = b[i-1]; b[i-1] = t;
            }
        } else { let t = a[i]; a[i] = b[i]; b[i] = t; }
    }
    res
}

// DP solution: input asure answer exists, which means:
// if a[i] <= b[i-1] or b[i] <= a[i-1], should keep (i-1)th and ith both swap or both not swap...
#[allow(dead_code)]
pub fn min_swap(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let (mut swap, mut stay) = (1, 0);
    for i in 1..a.len() {
        if a[i] <= b[i-1] || b[i] <= a[i-1] {
            swap += 1;  // swap i if i-1 swapped
        } else if a[i] <= a[i-1] || b[i] <= b[i-1] {
            let t = swap;
            swap = stay + 1; stay = t;
        } else {
            // swap or not both ok
            let min = if swap > stay { stay } else { swap };
            swap = min + 1; stay = min;
        }
    }
    if swap < stay { swap } else { stay }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins1 = vec![1,3,5,4];
        let ins2 = vec![1,2,3,7];
        assert_eq!(min_swap(ins1, ins2), 1);
    }
    #[test]
    fn it_works_02() {
        let ins1 = vec![0,7,8,10,10,11,12,13,19,18];
        let ins2 = vec![4,4,5,7,11,14,15,16,17,20];
        assert_eq!(min_swap(ins1, ins2), 4);
    }
}
