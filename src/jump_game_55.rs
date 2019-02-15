pub fn can_jump_1(nums: Vec<i32>) -> bool {
    use std::collections::VecDeque;
    if nums.len() == 0 { return true }

    // search the zero

    let mut dest = nums.len()-1;
    for idx in (dest..0).rev() {
        if nums[idx] == 0 { dest = idx; break; }
        if idx == 0 { return true }
    }
    let mut mark = vec![0;nums.len()];
    let mut deq : VecDeque<usize>= VecDeque::new();
    deq.push_back(0);
    mark[0] = 1;
    while let Some(idx) = deq.pop_front() {
        if nums[idx] as usize + idx >= dest { return true }
        let down = if idx > nums[idx] as usize { idx - nums[idx] as usize } else { 0 } ;
        for next in down..(1+idx+nums[idx] as usize) {
            if mark[next] == 1 { continue; }
            deq.push_back(next);
            mark[next] = 1;
        }
    }
    false
}

// much simple solution
// backtrace
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut last = nums.len()-1;
    for idx in (0..nums.len()-1).rev() {
        if idx + nums[idx] as usize >= last { last = idx; }
    }
    last == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(can_jump(vec![2,1,3,4,1,2,1,5,4]), true);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(can_jump(vec![2,3,1,1,4]), true);
    }
    #[test]
    fn it_works_3() {
        assert_eq!(can_jump(vec![3,2,1,0,4]), false);
    }
}
