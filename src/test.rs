
fn vec_test() {
    let input = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];
    println!("{:?}", input[0..9][1]);
}

use utils::ListNode;
use std::boxed::Box;
fn test_box() {
    let mut node = ListNode::new(1);
    let mut b1 = Box::new(&mut node);
    let mut b2 = b1;
    b2.next = Some(Box::new(ListNode::new(2)));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    fn it_works_01() {
        vec_test();
    }

    fn test_swap_01() {
        let (a, b) = (10, 100);
        // (a, b) = (b, a); // will compile error
        assert_eq!((a, b), (100, 10));
    }

    #[test]
    fn test_range() {
        for i in 10..9 {
            println!("range: {}.", i);
        }
    }
}
