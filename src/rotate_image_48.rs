// @url https://leetcode.com/problems/rotate-image/

// implace rotate
#[allow(dead_code)]
pub fn rotate_1(matrix: &mut Vec<Vec<i32>>) {
    if matrix.len() <= 1 { return }
    let (n, m) = (matrix.len(), matrix.len()-1);
    let mut marker : Vec<bool> = vec![false; n*n];
    for x in 0..n {
        for y in 0..n {
            if marker[x*n +y] { continue; }
            marker[x*n+y] = true;
            let (mut dest_x, mut dest_y) = (y, m-x);
            if dest_x == x && dest_y == y { continue; }

            let mut tmp = matrix[x][y];
            while ! marker[dest_x*n+dest_y] {
                let d_v = matrix[dest_x][dest_y];
                matrix[dest_x][dest_y] = tmp;
                tmp = d_v;
                marker[dest_x*n + dest_y] = true;
                let old_x = dest_x;
                dest_x = dest_y;
                dest_y = m-old_x;
            }
            matrix[x][y] = tmp;
        }
    }
}

// two swap
#[allow(dead_code)]
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    if n <=1 { return };
    for i in 0..n/2 {
        matrix.swap(i, n-1-i);
    }
    // another swap
    for i in 0..n {
        for j in i+1..n {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let mut image = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9]
        ];
        let res = vec![
            vec![7,4,1],
            vec![8,5,2],
            vec![9,6,3]
        ];
        rotate(&mut image);
        assert_eq!(image, res);
    }
    #[test]
    fn it_works_02() {
        let mut image = vec![
            vec![ 5, 1, 9,11],
            vec![ 2, 4, 8,10],
            vec![13, 3, 6, 7],
            vec![15,14,12,16]
        ];
        let res = vec![
            vec![15,13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7,10,11]
        ];
        rotate(&mut image);
        assert_eq!(image, res);
    }
}
