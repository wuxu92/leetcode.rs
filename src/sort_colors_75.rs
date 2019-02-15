// @url https://leetcode.com/problems/sort-colors/

// two-pass counting method
pub fn sort_colors_1(nums: &mut Vec<i32>) {
    let mut count = [0,0,0];
    for i in nums.iter() { count[*i as usize] += 1; }
    let mut idx = 0;
    for i in 0..3 {
        for _ in 0..count[i] { nums[idx] = i as i32; idx+=1;}
    }
}

// one-pass using swap and const space
pub fn sort_colors(nums: &mut Vec<i32>) {
    if nums.len() <= 1 { return }
    let mut l = 0;
    let mut h = nums.len()-1;
    while l < h {
        if nums[l] == 0 { l += 1; continue; }
        if nums[h] == 2 { h -= 1; continue; }
        let mut mid = l;
        while mid <= h {
            if nums[mid] == 0 { nums.swap(l, mid); l += 1; break;}
            if nums[mid] == 2 { nums.swap(mid, h); h -= 1; break;}
            mid += 1;
        }
        if mid > h { break; } // 没有需要交换的元素
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut nums = vec![2,0,2,1,1,0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0,0,1,1,2,2]);
    }
}

