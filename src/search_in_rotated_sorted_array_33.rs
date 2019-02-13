// @url https://leetcode.com/problems/search-in-rotated-sorted-array/
// improve: use len() instead of len()-1 as higher boundry
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut h = nums.len();
    let mut l = 0;
    while l < h {
        let mid = (l+h) / 2;
        println!("l: {}, h:{}, mid: {}, {} vs {} as mid: {}.", l, h, mid, nums[l], nums[h-1], nums[mid]);
        let item = nums[mid];
        let p0 = nums[l];
        if p0 == target { return l as i32 }
        if item == target { return mid as i32  }

        if p0 < target {    // left part
            if item < p0 || item > target { h = mid; }
            else { l = mid+1; }
        } else {        // right part
            if item > p0 || item < target { l = mid+1;}
            else { h = mid; }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 3), -1);
    }
    #[test]
    fn it_works_3() {
        assert_eq!(search(vec![4,5,6,7,8,9,100], 9), 5);
    }
    #[test]
    fn it_works_4() {
        assert_eq!(search(vec![4,5,6,7,8,9,100], 100), 6);
    }
    #[test]
    fn it_works_5() {
        assert_eq!(search(vec![4,5,6,7,8,9,100], 4), 0);
    }
    #[test]
    fn it_works_6() {
        assert_eq!(search(vec![], 5), -1);
    }
    #[test]
    fn it_works_7() {
        assert_eq!(search(vec![1, 3], 0), -1);
    }
    #[test]
    fn it_works_8() {
        assert_eq!(search(vec![4,5,6,7,8,9,11, 20, 100], 8), 4);
    }
    #[test]
    fn it_works_9() {
        assert_eq!(search(vec![ 100, 4,5,6,7,8,9,11, 20], 8), 5);
    }
    #[test]
    fn it_works_10() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 6), 2);
    }
}
