// @url https://leetcode.com/problems/binary-tree-level-order-traversal/

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use utils::{TreeNode,build_tree};

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res : Vec<Vec<i32>> = vec![];
    fn process(que: &Vec<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>) {
        if que.len() == 0 { return };
        let mut nvec : Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut rvec : Vec<i32> = Vec::new();
        for node in que {
            rvec.push(node.borrow().val);
            if let Some(lnode) = node.borrow().left.clone(){
                nvec.push(Rc::clone(&lnode));
            }
            if let Some(rnode) = node.borrow().right.clone(){
                nvec.push(Rc::clone(&rnode));
            }
        }
        res.push(rvec);
        process(&nvec, res);
    }
    if let Some(node) = root { process(&vec![node], &mut res); }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let null = std::i32::MIN;
        let input = build_tree(vec![3,9,20,null,null,15,7]);
        assert_eq!(level_order(input), vec![
                   vec![3],
                   vec![9,20],
                   vec![15,7]
        ]);
    }
}
