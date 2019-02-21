// @url https://leetcode.com/problems/course-schedule/

// recursive map version, may exceed time limit
#[allow(dead_code)]
pub fn can_finish_1(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    if prerequisites.len() == 0 { return true }
    use std::collections::BTreeMap;
    let mut map : BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    let mut prune : Vec<bool> = vec![false; num_courses as usize];

    for vec in prerequisites.iter() {
        let (l, r) = (vec[0], vec[1]);
        if ! map.contains_key(&l) { map.insert(l , vec![r]); }
        else { map.get_mut(&l).unwrap().push(r); }
    }
    // println!("map: {:?}.", map);
    let mut res = true;
    fn process(m: &BTreeMap<i32, Vec<i32>>, prune: &mut Vec<bool>, v: Vec<i32>, t: i32, res: &mut bool) {
        if ! m.contains_key(&t) || prune[t as usize] { return }
        let vec = m.get(&t).unwrap();
        let mut new_vec = v.clone();
        new_vec.push(t);
        for item in vec.iter() {
            if v.contains(item) { *res = false; return }
            if prune[*item as usize] { continue }
            process(m, prune, new_vec.clone(), *item, res);
            if *res == false { return }
        }
        prune[t as usize] = true;
    }
    // prepare full vec
    for i in 0..num_courses {
        if ! map.contains_key(&i) || prune[i as usize] { continue }
        let start = vec![i];
        for val in map.get(&i).unwrap() {
            process(&map, &mut prune, start.clone(), *val, &mut res);
        }
    }
    res
}

// DFS
#[allow(dead_code)]
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    if prerequisites.len() == 0 { return true }
    let num = num_courses as usize;
    let mut grid : Vec<Vec<bool>> = vec![];
    for _i in 0..num_courses { grid.push(vec![false; num]); }
    for v in prerequisites.iter() {
        grid[v[0] as usize][v[1] as usize] = true;
    }
    let mut marker : Vec<i8> = vec![0; num];

    // return true if found round
    fn process(grid: &mut Vec<Vec<bool>> , marker : &mut Vec<i8>, row: usize) -> bool {
        marker[row] = 1; // 访问过
        for col in 0..grid[0].len() {
            if grid[row][col] {
                if marker[col] == 1 { return true }
                if marker[col] == -1 { continue }
                // if grid[col][row] { return true }
                if process(grid, marker, col) { return true }
            }
        }
        marker[row] = -1;
        false
    }
    for i in 0..num {
        if process(&mut grid, &mut marker, i) { return false }
    }
    true
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(can_finish(2, vec![vec![1,0]]), true);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(can_finish(2, vec![vec![1,0], vec![0,1]]), false);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(can_finish(4, vec![vec![0, 1], vec![1,3], vec![3,1]]), false);
    }
    #[test]
    fn it_works_04() {
        assert_eq!(can_finish(4, vec![
                              vec![2,0],vec![1,0],vec![3,1],vec![3,2],vec![1,3]
        ]), false);
    }
}
