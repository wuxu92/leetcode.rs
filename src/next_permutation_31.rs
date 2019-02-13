// @url https://leetcode.com/problems/next-permutation/
pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut h = nums.len()-1;
    // no need infact
    // let f = |x: i32, y: i32| -> bool {
    //     x.to_string() > y.to_string()
    // };
    while h > 0 {
        // if f(nums[h], nums[h-1]) { break; }
        if nums[h] > nums[h-1] { break; }
        h -= 1;
    }
    if h == 0 {
        nums.reverse();
        return 
    }
    // search bigger one
    let mut ex = h-1;
    h = nums.len()-1;
    while h > ex {
        if nums[h] > nums[ex] { nums.swap(ex, h); break; }
        h -= 1;
    }
    // reverse after ex
    h = nums.len()-1;
    ex = ex + 1;
    while ex < h {
        nums.swap(ex, h);
        ex += 1; h-=1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut left = vec![1,2,3];
        let r = vec![1,3,2];
        next_permutation(&mut left);
        assert_eq!(left, r);
    }
    #[test]
    fn it_works_2() {
        let mut left = vec![1, 3, 2];
        let r = vec![2, 1, 3];
        // let r = vec![2, 3,  1];
        next_permutation(&mut left);
        assert_eq!(left, r);
    }
    #[test]
    fn it_works_3() {
        let mut left = vec![3, 21, 1];
        let r = vec![1, 21, 3];
        next_permutation(&mut left);
        assert_eq!(left, r);
    }
    #[test]
    fn it_works_4() {
        let mut left = vec![1, 1, 5];
        let r = vec![1, 5, 1];
        next_permutation(&mut left);
        assert_eq!(left, r);
    }
    #[test]
    fn it_works_5() {
        let mut left = vec![1,3,5,1,2,4];
        let r = vec![1,3,5,1,4,2];
        next_permutation(&mut left);
        assert_eq!(left, r);
    }
    #[test]
    fn it_works_6() {
        let mut left = vec![2, 3, 1];
        next_permutation(&mut left);
        assert_eq!(left, vec![3, 1, 2]);
    }
    #[test]
    fn it_works_7() {
        let mut left = vec![4,2,0,2,3,2,0];
        next_permutation(&mut left);
        assert_eq!(left, vec![4,2,0,3,0,2,2]);
    }
    #[test]
    fn it_works_8() {
        let mut left = vec![4,24,4,17,26,3,21,17,25,9,16,22,11,17,28,5,17,24,1,8,25,29,7,15,13,8,8,3,21,18,9,26,4,13,13,23,8,26,10,4,28,18,18,9,27,17,6,14,3,0,23,20,29,22];
        next_permutation(&mut left);
        assert_eq!(left, vec![4,24,4,17,26,3,21,17,25,9,16,22,11,17,28,5,17,24,1,8,25,29,7,15,13,8,8,3,21,18,9,26,4,13,13,23,8,26,10,4,28,18,18,9,27,17,6,14,3,0,23,22,20,29]);
    }
}
