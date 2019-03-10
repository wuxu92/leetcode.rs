// @url https://leetcode.com/problems/kth-largest-element-in-an-array/

#[allow(dead_code)]
pub fn find_kth_largest_01(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable_by(|a, b| b.cmp(a));
    nums[k as usize - 1]
}

// quick select  @url https://en.wikipedia.org/wiki/Quickselect
#[allow(dead_code)]
pub fn find_kth_largest_2(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;

    fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
        if left >= right { return left }
        let pivot = (left + right) / 2;
        nums.swap(pivot, right);
        let mut first_large = left;
        for idx in left..right {
            if nums[idx] < nums[right] {
                nums.swap(idx, first_large);
                first_large += 1;
            }
        }
        nums.swap(first_large, right);
        first_large
    }

    fn select(nums: &mut Vec<i32>, left: usize, right: usize, k: usize) -> i32 {
        let p = partition(nums, left, right);
        if p == k { return nums[p] }
        if p > k {
            return select(nums, left, p-1, k);
        }
        return select(nums, p+1, right, k);
    }
    let len = nums.len();
    select(&mut nums, 0, len-1, len - k as usize) // (len-k)-th small
}

// use min heap, available when nums is very large
#[allow(dead_code)]
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    use std::collections::BinaryHeap;
    let mut heap : BinaryHeap<i32> = BinaryHeap::new();
    for i in 0..k {
        heap.push(-1 * nums[i]);
    }
    for i in k..nums.len() {
        let val = -1 * nums[i];
        if val < *heap.peek().unwrap() {
            heap.pop();
            heap.push(val);
        }
    }
    *heap.peek().unwrap() * -1
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![3,2,1,5,6,4];
        assert_eq!(find_kth_largest(ins, 2), 5);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![3,2,3,1,2,4,5,5,6];
        assert_eq!(find_kth_largest(ins, 4), 4);
    }
}
