// @url https://leetcode.com/problems/4sum/

// use loop  
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res : Vec<Vec<i32>> = Vec::new();
    if nums.len() < 4 { return res };
    let mut nums = nums.clone();
    let check_out = |sum:i32| -> bool {
        target >=0 && sum > target
    };

    nums.sort_unstable();
    for i in 0..nums.len()-3 {
        let sum = nums[i];
        if check_out(sum) { break; }
        for j in i+1..nums.len()-2 {
            let sum = sum + nums[j];
            if check_out(sum) { break; }
            for k in j+1..nums.len()-1 {
                let sum = sum + nums[k];
                if check_out(sum) { break; }
                for l in k+1..nums.len() {
                    let sum = sum + nums[l];
                    if sum == target {
                        // check if dup
                        let item = vec![nums[i], nums[j], nums[k], nums[l]];
                        if res.contains(&item) { continue; }
                        res.push(item);
                    }
                    if check_out(sum) { break; }
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(four_sum(vec![1, 0, -1, 0, -2, 2], 0), vec![
                   vec![-1,  0, 0, 1],
                   vec![-2, -1, 1, 2],
                   vec![-2,  0, 0, 2]
        ]);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(four_sum(vec![1, 0, -1, 0, -2, 2], 3), vec![
                   vec![0,  0, 1, 2]
        ]);
    }
    #[test]
    fn it_works_3() {
        assert_eq!(four_sum(vec![1, 0, -1, 0], 0), vec![
                   vec![-1,  0, 0, 1],
        ]);
    }
    #[test]
    fn it_works_4() {
        let res: Vec<Vec<i32>> = Vec::new();
        assert_eq!(four_sum(vec![1, 0, -1, 0, -2, 2], 30), res);
    }
    #[test]
    fn it_works_5() {
        assert_eq!(four_sum(vec![1, 0, -1, -1, -1, -1, -1], -4), vec![
                   vec![-1, -1, -1, -1]
        ]);
    }
    #[test]
    fn it_works_6() {
        assert_eq!(four_sum(vec![-5,-4,-3,-2,-1,0,0,1,2,3,4,5], 0), vec![
                   vec![-5,-4,4,5],vec![-5,-3,3,5],vec![-5,-2,2,5],vec![-5,-2,3,4],vec![-5,-1,1,5],vec![-5,-1,2,4],vec![-5,0,0,5],vec![-5,0,1,4],vec![-5,0,2,3],vec![-4,-3,2,5],vec![-4,-3,3,4],vec![-4,-2,1,5],vec![-4,-2,2,4],vec![-4,-1,0,5],vec![-4,-1,1,4],vec![-4,-1,2,3],vec![-4,0,0,4],vec![-4,0,1,3],vec![-3,-2,0,5],vec![-3,-2,1,4],vec![-3,-2,2,3],vec![-3,-1,0,4],vec![-3,-1,1,3],vec![-3,0,0,3],vec![-3,0,1,2],vec![-2,-1,0,3],vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]
        ]);
    }
}
