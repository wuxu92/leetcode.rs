// @url https://leetcode.com/problems/subsets/

// recursive version
pub fn subsets_1(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res : Vec<Vec<i32>> = vec![vec![]];
    // sort nums
    let mut nums = nums.clone();
    nums.sort_unstable();
    fn process(
        res: &mut Vec<Vec<i32>>,
        set:Vec<i32>,
        nums: &mut Vec<i32>)
    {
        while nums.len() > 0 {
            let i = nums.remove(0);
            let mut v = set.clone();
            v.push(i);
            let vv = v.clone();
            res.push(v);
            process(res, vv, &mut nums.clone())
        }
    }
    process(&mut res, vec![], &mut nums);
    res
}

// bfs no recursive
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res : Vec<Vec<i32>> = vec![vec![]];
    for item in nums {
        for i in 0..res.len() {
            let mut v = res[i].clone();
            v.push(item);
            res.push(v);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(subsets(vec![1,2,3]), 
				   vec![
				   vec![3],
				   vec![1],
				   vec![2],
				   vec![1,2,3],
				   vec![1,3],
				   vec![2,3],
				   vec![1,2],
				   vec![]
				   ]
				  );
    }
}
