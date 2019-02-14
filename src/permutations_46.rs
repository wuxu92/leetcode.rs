
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res:Vec<Vec<i32>> = Vec::new();
    fn recursive_push (res: &mut Vec<Vec<i32>>, repo: Vec<i32>, items: Vec<i32>) {
        if items.len() == 0 { res.push(repo.clone()); return }

        for (idx, &item) in items.iter().enumerate() {
            let mut new_vec = items.clone();
            new_vec.remove(idx);
            let mut repo2 = repo.clone();
            repo2.push(item);
            recursive_push(res, repo2, new_vec);
        }
    };
    if nums.len() != 0 {recursive_push(&mut res, Vec::new(), nums);}
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(permute(vec![1,2,3]), vec![
                     vec![1,2,3],
                     vec![1,3,2],
                     vec![2,1,3],
                     vec![2,3,1],
                     vec![3,1,2],
                     vec![3,2,1]
        ]);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(permute(vec![1]), vec![ vec![1] ]);
    }
    #[test]
    fn it_works_3() {
        assert_eq!(permute(vec![1,2]), vec![
                     vec![1,2],
                     vec![2,1]
        ]);
    }
    #[test]
    fn it_works_4() {
        let res: Vec<Vec<i32>> = Vec::new();
        assert_eq!(permute(vec![]),res); 
    }
}
