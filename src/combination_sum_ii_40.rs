// @url https://leetcode.com/problems/combination-sum-ii/

#[allow(dead_code)]
pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candi = candidates;
    candi.sort_unstable();

    let mut res : Vec<Vec<i32>> = Vec::new();
    fn process(res: &mut Vec<Vec<i32>>, cur: Vec<i32>, t: i32, items: &[i32]) {
        if items.len() == 0 || items[0] > t { return }

        for idx in 0..items.len() {
            let val = items[idx];
            if idx > 0 && val == items[idx-1] { continue; }
            if val > t { return; }

            let mut vec = cur.clone();
            vec.push(val);
            if val == t { res.push(vec); break; }
            // recursive
            process(res,  vec, t-val, &items[idx+1..]);
        }
    }
    process(&mut res, Vec::new(), target, &candi[..]);
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(combination_sum2(vec![10,1,2,7,6,1,5], 8), vec![
                   vec![1, 7],
                   vec![1, 2, 5],
                   vec![2, 6],
                   vec![1, 1, 6]
        ]);
    }
}
