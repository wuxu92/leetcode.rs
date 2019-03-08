// @url https://leetcode.com/problems/longest-consecutive-sequence/

// use a vector vec![s,e,s,e,s,e...]
#[allow(dead_code)]
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0 }
    let mut vec : Vec<i32> = vec![];
    for n in nums {
        let (mut idx, mut flag) = (0, false);
        while idx < vec.len() {
            let (s, e) = (vec[idx], vec[idx+1]);
            if n >= s && n <= e { flag = true; break; }
            if n == s-1 {
                vec[idx] = n; flag = true;
                // merge
                if idx > 0 && vec[idx-1] == n-1 {
                    vec[idx-1] = e;
                    vec.remove(idx);
                    vec.remove(idx);
                }
                break;
            }
            if n == e+1 {
                vec[idx+1] = n; flag = true;
                if idx +3 < vec.len() && vec[idx+2] == n+1 {
                    vec[idx+2] = vec[idx];
                    vec.remove(idx);
                    vec.remove(idx);
                }
                break;
            }
            if n < s {
                vec.insert(idx, n);
                vec.insert(idx, n);
                flag = true;
                break;
            }
            idx += 2;
        }
        if !flag { vec.push(n); vec.push(n); }
        // println!("n: {}, vec: {:?}", n, vec);
    }
    // println!("vec: {:?}", vec);
    let (mut idx, mut len) = (0, 0);
    while idx < vec.len() {
        let tmp = vec[idx+1] - vec[idx];
        if tmp > len { len = tmp; }
        idx += 2;
    }
    len + 1
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(ins), 4);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![100, 4, 2, 1, 3, 2];
        assert_eq!(longest_consecutive(ins), 4);
    }
    #[test]
    fn it_works_03() {
        let ins = vec![5,10,20,0, 100, 4, 2, 1, 3, 2];
        assert_eq!(longest_consecutive(ins), 6);
    }
    #[test]
    fn it_works_04() {
        let ins = vec![2];
        assert_eq!(longest_consecutive(ins), 1);
    }
    #[test]
    fn it_works_05() {
        let ins = vec![];
        assert_eq!(longest_consecutive(ins), 0);
    }
    #[test]
    fn it_works_06() {
        let ins = vec![2,3,4,5,6,7];
        assert_eq!(longest_consecutive(ins), 6);
    }
}
