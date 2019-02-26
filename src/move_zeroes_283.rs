// @url https://leetcode.com/problems/move-zeroes/

#[allow(dead_code)]
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut zcount = 0;
    for idx in 0..nums.len() {
        // println!("idx:{}, count:{}, num:{}, nums:{:?}", idx, zcount, nums[idx], nums);
        if nums[idx] == 0 { zcount += 1; }
        else {
            nums.swap(idx-zcount, idx);
        }
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let mut s = vec![0,1,0,3,12];
        move_zeroes(&mut s);
        assert_eq!(s, vec![1,3,12,0,0]);
    }
}
