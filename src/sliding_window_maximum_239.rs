// @url https://leetcode.com/problems/sliding-window-maximum/
// You may assume k is always valid, 1 ≤ k ≤ input array's size for non-empty array

#[allow(dead_code)]
pub fn max_sliding_window_1(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.len() == 0 || k == 0 { return vec![] }
    let (mut res, len) = (nums.clone(), nums.len());
    for i in 0..k as usize - 1 {
        for j in 0..len-i-1 {
            if res[j] < res[j+1] { res[j] = res[j+1]; }
        }
    }
    for _i in 0..k-1 {
        res.pop();
    }
    res
}

// two pass solution
#[allow(dead_code)]
pub fn max_sliding_window_2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let len = nums.len();
    if len == 0 || k == 0 { return vec![] }
    let (mut left, mut right) = (vec![0;nums.len()], vec![0;nums.len()]);
    let (k, mut res) = (k as usize, vec![]);
    res.reserve(len-k+1);
    left[0] = nums[0]; right[len-1] = nums[len-1];
    for i in 1..len {
        let j = len - 1 - i;
        if i % k == 0 { 
            left[i] = nums[i];
        } else {
            left[i] = if nums[i]>left[i-1] {nums[i]} else {left[i-1]};
        }
        if j % k == 0 {
            right[j] = nums[j];
        } else {
             right[j] = if nums[j]>right[j+1] {nums[j]} else {right[j+1]};
         }
    }
    for i in 0..len-k+1 {
        res.push(if right[i]>left[i+k-1] {right[i]} else {left[i+k-1]});
    }
    res
}

// two pass solution, speed up
#[allow(dead_code)]
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let len = nums.len();
    if len == 0 || k == 0 { return vec![] }
    let (mut left, mut right) = (nums.clone(), nums.clone());
    let (k, mut res) = (k as usize, vec![]);
    res.reserve(len-k+1);
    left[0] = nums[0]; right[len-1] = nums[len-1];

    for i in 1..len {
        let j = len - 1 - i;
        if i % k != 0 && nums[i]<left[i-1] {left[i] = left[i-1]};
        if j % k != 0 && nums[j]<right[j+1] { right[j] = right[j+1]};
    }
    for i in 0..len-k+1 {
        res.push(if right[i]>left[i+k-1] {right[i]} else {left[i+k-1]});
    }
    res
}

// TODO use deque

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(
            max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3),
            vec![3,3,5,5,6,7]
        );
    }
    #[test]
    fn it_works_02() {
        assert_eq!(
            max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 1),
            vec![1,3,-1,-3,5,3,6,7]
        );
    }
    #[test]
    fn it_works_03() {
        assert_eq!(
            max_sliding_window(vec![1,3,-1], 3),
            vec![3]
        );
    }
    #[test]
    fn it_works_04() {
        assert_eq!(
            max_sliding_window(vec![], 0),
            vec![]
        );
    }
    #[test]
    fn it_works_05() {
        assert_eq!(
            max_sliding_window(vec![9,10,9,-7,-4,-8,2,-6], 5),
            vec![10,10,9,2]
        );
    }
    use std::fs::File;
    use std::io::prelude::*;
    #[test]
    fn it_works_06() {
        let mut f = File::open("/home/wuxu/tmp/input.txt").expect("File not found!");
        let mut content = String::new();
        f.read_to_string(&mut content).expect("Err on reading file!");
        let ins_str : Vec<&str> = content.split(",").collect();
        let mut ins : Vec<i32> = vec![];
        ins.reserve(ins_str.len());
        for s in ins_str {
            match i32::from_str_radix(s, 10) {
                Ok(num) => ins.push(num),
                Err(why) => println!("convert {} to num failed.", s)
            }
        }
        let res = max_sliding_window(ins, 100);
        // write reults
        let mut f = File::create("/home/wuxu/tmp/result.txt").expect("Create result file failed.");
        write!(f, "{:?}", res);

        let mut f = File::open("/home/wuxu/tmp/a.txt").expect("File not found!");
        let mut content = String::new();
        f.read_to_string(&mut content).expect("Err on reading file!");
        let ins_str : Vec<&str> = content.split(",").collect();
        let mut out : Vec<i32> = vec![];
        out.reserve(ins_str.len());
        for s in ins_str {
            match i32::from_str_radix(s, 10) {
                Ok(num) => out.push(num),
                Err(why) => println!("convert {} to num failed.", s)
            }
        }
        println!("expect: {}, res: {}", out.len(), res.len());
        assert_eq!(res, out);
        let mut f = File::create("/home/wuxu/tmp/expect.txt").expect("Create result file failed.");
        write!(f, "{:?}", out);

    }

}
