// @url https://leetcode.com/problems/majority-element/

// quick sort
#[allow(dead_code)]
pub fn majority_element_1(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums[nums.len()/2]
}

// iteration , for more the half has value 
#[allow(dead_code)]
pub fn majority_element_2(nums: Vec<i32>) -> i32 {
    let (mut t, mut cnt) = (0, 0);
    for num in nums {
        if cnt == 0 { t = num; cnt = 1; }
        else if num == t { cnt += 1; }
        else { cnt -= 1; }
    }
    t
}

#[allow(dead_code)]
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut vec = vec![];
    for num in nums {
        let l = vec.len();
        if l == 0 { vec.push(num); continue }
        else {
            if vec[l-1] == num { vec.push(num); }
            else { vec.pop(); }
        }
    }
    vec[0]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(majority_element(vec![3,2,3]), 3);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(majority_element(vec![2,2,1,1,1,2,2]), 2);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(majority_element(vec![2,2,2,2,1,2,2]), 2);
    }
    #[test]
    fn it_works_04() {
        assert_eq!(majority_element(vec![2]), 2);
    }
    #[test]
    fn it_works_05() {
        assert_eq!(majority_element(vec![1,2,3,4,1,1,1,10,10,10,10,10,10,10,10,2]), 10);
    }
}
