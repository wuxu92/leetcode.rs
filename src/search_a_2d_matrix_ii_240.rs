// @url https://leetcode.com/problems/search-a-2d-matrix-ii/

// no rust avalible in leetcode
#[allow(dead_code)]
fn search_matrix(m: Vec<Vec<i32>>, target: i32) -> bool {
    if m.len() == 0 || m[0].len() == 0 { return false }
    let (mut r, mut l) = (0, m[0].len() - 1);
    use std::cmp::Ordering;
    while r < m.len() {
        match m[r][l].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => { r += 1; },
            Ordering::Greater => {
                if l == 0 { break;}
                l -= 1;
            }
        };
    }
    false
}

// an accepted cpp solution
// bool searchMatrix(vector<vector<int>>& matrix, int target) {
//     if (matrix.size() == 0 || matrix[0].size() == 0) { return false; }
//     int r = matrix.size(), l = matrix[0].size();
//     int x = 0, y = l-1;
//     while (x<r && y>=0) {
//         if (matrix[x][y] == target) { return true; }
//         if (matrix[x][y] > target) { y--; }
//         else { x++; }
//     }
//     return false;
// }

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let ins = vec![
            vec![1,   4,  7, 11, 15],
            vec![2,   5,  8, 12, 19],
            vec![3,   6,  9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30]
        ];
        assert_eq!(search_matrix(ins, 5), true);
    }
    #[test]
    fn it_works_02() {
        let ins = vec![
            vec![1,   4,  7, 11, 15],
            vec![2,   5,  8, 12, 19],
            vec![3,   6,  9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30]
        ];
        assert_eq!(search_matrix(ins, 20), false);
    }
}
