
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0 }
    let mut cur = 0;
    let mut cur_max = nums[0];
    for var in nums {
        let tmp = cur + var;
        if tmp > cur_max { cur_max = tmp; }
        if tmp < 0 { cur = 0 ; continue; }
        else { cur = tmp; }
    }
    cur_max
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(max_sub_array(vec![-2,-3,-1,-5,4]), 4);
    }
    #[test]
    fn it_works_3() {
        assert_eq!(max_sub_array(vec![-2,-3,-1,-5]), -1);
    }
    #[test]
    fn it_works_4() {
        assert_eq!(max_sub_array(vec![-3,-2, 0,-1]), 0);
    }
}
