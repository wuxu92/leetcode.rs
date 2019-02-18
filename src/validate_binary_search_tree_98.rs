// @url https://leetcode.com/problems/validate-binary-search-tree/

// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[allow(dead_code)]
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

// Wrong version
#[allow(dead_code)]
pub fn is_valid_bst_1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn process(node: &Rc<RefCell<TreeNode>>) -> bool {
        // let cell = &node;
        let val = node.borrow().val;
        let left = node.borrow().left.clone();
        if let Some(lcell) = left {
            let lval = lcell.borrow().val;
            if lval >= val { return false }
            if ! process(&lcell) { return false }
        }
        let right = node.borrow().right.clone();
        if let Some(rcell) = right {
            let rval = rcell.borrow().val;
            if rval <= val {return false};
            if ! process(&rcell) { return false }
        }
        true
    }
    if let Some(node) = root { return process(&node) }
    true
}

// success solution
#[allow(dead_code)]
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut last = std::i32::MIN;
    let mut init = false;
    fn process(node: &Option<Rc<RefCell<TreeNode>>>, last: &mut i32, init: &mut bool) -> bool {
        if let Some(item) = node {
            if ! process(&item.borrow().left, last, init) { return false }
            let val = item.borrow().val;
            if ! *init { *init = true; }
            else if *last >= val { return false }
            *last = val;
            if ! process(&item.borrow().right, last, init) {return false}
        }
        true
    }
    return process(&root, &mut last, &mut init)
}

// build tree by vector, use i32::MIN as null
#[allow(dead_code)]
pub fn build_tree(vec: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let res = Rc::new(RefCell::new(TreeNode::new(vec[0])));
    let ret = Rc::clone(&res);
    let mut idx = 0;
    fn process(rc: Rc<RefCell<TreeNode>>, vec: &[i32], idx: &mut usize) {
        *idx += 1;
        if *idx >= vec.len() { return }
        if vec[*idx] != std::i32::MIN {
            let node = Rc::new(RefCell::new(TreeNode::new(vec[*idx])));
            // rc.borrow_mut();
            rc.borrow_mut().left = Some(Rc::clone(&node));
            process(node, vec, idx);
        }
        *idx+=1;
        if *idx >= vec.len() { return }
        if vec[*idx] != std::i32::MIN {
            let node = Rc::new(RefCell::new(TreeNode::new(vec[*idx])));
            rc.borrow_mut().right = Some(Rc::clone(&node));
            process(node, vec, idx);
        }
    };
    process(res, &vec[..], &mut idx);
    Some(ret)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let min = std::i32::MIN;
        let input = build_tree(vec![2,1,min,min,3]);
        assert_eq!(is_valid_bst(input), true);
    }
    #[test]
    fn it_works_02() {
        let min = std::i32::MIN;
        let input = build_tree(vec![1,1]);
        assert_eq!(is_valid_bst(input), false);
    }
    #[test]
    fn it_works_03() {
        let min = std::i32::MIN;
        let input = build_tree(vec![10,5,15,min,min,6,20]);
        assert_eq!(is_valid_bst(input), false);
    }
    #[test]
    fn it_works_04() {
        let min = std::i32::MIN;
        let input = build_tree(vec![5,1,4,min,min,3,6]);
        assert_eq!(is_valid_bst(input), false);
    }
    #[test]
    fn it_works_05() {
        let min = std::i32::MIN;
        let input = build_tree(vec![5,1,4,min,min,3,6]);
        assert_eq!(is_valid_bst(input), false);
    }
    #[test]
    fn it_works_06() {
        let min = std::i32::MIN;
        let input = build_tree(vec![-2147483648]);
        assert_eq!(is_valid_bst(input), true);
    }
}
