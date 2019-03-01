// @url https://leetcode.com/problems/queue-reconstruction-by-height/

#[allow(dead_code)]
pub fn reconstruct_queue_1(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut pp = people;
    pp.sort_unstable_by(|a, b| {a[1].cmp(&b[1])});
    let mut res : Vec<Vec<i32>> = vec![];
    for vec in pp {
        let mut cnt = 0;
        for idx in 0..res.len() {
            if res[idx][0] >= vec[0] {
                if cnt >= vec[1] {
                    res.insert(idx, vec.clone());
                    cnt = -1; break;
                } else { cnt += 1; }
            }
        }
        if cnt >= 0 { res.push(vec); }
    }
    res
}

#[allow(dead_code)]
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (mut res, mut pp) = (Vec::new(), people);
    if pp.len() == 0 { return res }
    pp.sort_unstable_by(|a, b| {
        if a[0] == b[0] { a[1].cmp(&b[1]) } else { b[0].cmp(&a[0]) }
    });
    res.reserve(pp.len());

    for p in 0..pp.len(){
        res.insert(pp[p][1] as usize, pp[p].clone());
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]];
        let out = vec![vec![5,0], vec![7,0], vec![5,2], vec![6,1], vec![4,4], vec![7,1]];
        assert_eq!(reconstruct_queue(ins), out);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![vec![0,0],vec![6,2],vec![5,5],vec![4,3],vec![5,2],vec![1,1],vec![6,0],vec![6,3],vec![7,0],vec![5,1]];
        let out = vec![vec![0, 0], vec![6, 0], vec![1, 1], vec![5, 1], vec![5, 2], vec![4, 3], vec![7, 0], vec![6,2], vec![5, 5], vec![6, 3]];
        assert_eq!(reconstruct_queue(ins), out);
    }
}
