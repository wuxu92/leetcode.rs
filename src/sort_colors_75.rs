// @url https://leetcode.com/problems/sort-colors/

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut count = [0,0,0];
    for i in nums.iter() { count[*i as usize] += 1; }
    let mut idx = 0;
    for i in 0..3 {
        for _ in 0..count[i] { nums[idx] = i as i32; idx+=1;}
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

