
pub fn bsearch<F>(nums: &Vec<i32>, lo: usize, hi: usize, t:i32, f: F) ->i32
    where F: Fn(i32, i32) -> bool {
    let mut hi = hi;
    let mut lo = lo;
    while lo < hi {
        let mid = (lo + hi) / 2;
        let item = nums[mid];
        println!("search: {}:{}:{}:{}:{}", lo, hi, mid, nums[lo], nums[hi-1]);
        let mut item2 = item;
        if mid < hi -1 { item2 = nums[mid+1]};
        if f(item, item2) {return mid as i32};
        if item > t { hi = mid; }
        else { lo = mid + 1;}
    }
    -1
}

// Wrong
pub fn search_range_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // search should after proper postion
    let midd = bsearch(&nums, 0, nums.len(), target, |item1, _| -> bool {item1 == target});
    println!("midd as : {}.", midd);
    let mut res = vec![-1, -1];
    if midd == -1 { return res }
    // search boundry
    let midd = midd as usize;
    let low = bsearch(&nums, 0, midd, target, |item1, item2| -> bool {item1<target && item2 >= target});
    let hi = bsearch(&nums, midd+1, nums.len(), target, |item1, item2| -> bool {item1<target && item2 >= target});
    if low == -1 { res[0] = 0 } else {res[0] = low; }
    if hi == -1 { res[1] = nums.len() as i32 } else {res[1] = hi; }

    res
}
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = vec![-1, -1];
    if nums.len() == 0 { return res };
    let mut hi = nums.len()-1;
    let mut lo = 0;
    while lo < hi {
        if lo+1 == hi {
            if nums[lo] == target { res[0] = lo as i32; }
            else if nums[hi] == target { res[0] = hi as i32; }
            break;
        }
        let mid = (hi+lo)/2;
        if nums[mid] >= target { hi = mid; }
        else { lo = mid ; }
    }
    hi = nums.len()-1;
    lo = 0;
    while lo < hi {
        if lo+1 == hi {
            if nums[hi] == target { res[1] = hi as i32; }
            else if nums[lo]==target { res[1] = lo as i32; }
            break;
        }
        let mid = (hi+lo)/2;
        if nums[mid] <= target { lo = mid; }
        else { hi = mid ; }
    }
    if nums.len()==1 && nums[0] == target { res = vec![0,0];}
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(search_range(vec![5,7,7,8,8,10], 8), vec![3,4]);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(search_range(vec![5,7,7,8,8,10], 1), vec![-1,-1]);
    }
    #[test]
    fn it_works_3() {
        assert_eq!(search_range(vec![5,7,7,8,8,10], 10), vec![5,5]);
    }
    #[test]
    fn it_works_4() {
        assert_eq!(search_range(vec![], 1), vec![-1,-1]);
    }
    #[test]
    fn it_works_5() {
        assert_eq!(search_range(vec![1], 1), vec![0,0]);
    }
    #[test]
    fn it_works_6() {
        assert_eq!(search_range(vec![5,7,7,7,7,8,8,8,8,8], 8), vec![5,9]);
    }
    #[test]
    fn it_works_7() {
        assert_eq!(search_range(vec![5,7,7,8,8,10], 5), vec![0,0]);
    }
    #[test]
    fn it_works_8() {
        assert_eq!(search_range(vec![5,5,7,7,8,8,10], 5), vec![0,1]);
    }
    #[test]
    fn it_works_9() {
        assert_eq!(search_range(vec![5,7,7,8,8,10], 6), vec![-1,-1]);
    }
}
