// @url https://leetcode.com/problems/minimum-size-subarray-sum/

#[allow(dead_code)]
pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    let (mut start, mut res) = (0, nums.len()+1);

    let mut cur = 0;
    for i in 0..nums.len() {
        if nums[i] >= s { return 1 }
        cur += nums[i];
        if cur >= s {
            while start < i {
                cur -= nums[start];
                if cur < s {
                    if i - start + 1 < res { res = i - start + 1; }
                    start += 1;
                    break;
                }
                start += 1;
            }
        }
    }
    if res == nums.len() + 1 { 0 } else {res as i32}
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![2,3,1,2,4,3];
        assert_eq!(min_sub_array_len(7, ins), 2);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![2,3,8,2,4,3];
        assert_eq!(min_sub_array_len(7, ins), 1);
    }
    #[test]
    fn it_works_03() {
        let ins = vec![2,3,5,2,4,3];
        assert_eq!(min_sub_array_len(7, ins), 2);
    }
    #[test]
    fn it_works_04() {
        let ins = vec![1,1,1,2,2,3];
        assert_eq!(min_sub_array_len(7, ins), 3);
    }
    #[test]
    fn it_works_05() {
        let ins = vec![1,1];
        assert_eq!(min_sub_array_len(3, ins), 0);
    }
    #[test]
    fn it_works_06() {
        let ins = vec![1,4,4];
        assert_eq!(min_sub_array_len(4, ins), 1);
    }
}
