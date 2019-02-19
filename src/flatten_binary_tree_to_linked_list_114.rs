// @url https://leetcode.com/problems/flatten-binary-tree-to-linked-list/

use std::rc::Rc;
use std::cell::RefCell;
use utils::TreeNode;

#[allow(dead_code)]
pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {

    fn process(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        // if let Some(ref left) = node.borrow().left {
        {
            let mut node = root.borrow_mut();
            if node.left.is_some() {
                let lr = process(Rc::clone(node.left.as_ref().unwrap()));
                // println!("process: node:{:?}, lr: {:?}\n", node, lr);
                lr.borrow_mut().right = node.right.clone();
                node.right = node.left.clone();
                node.left = None;
                // println!("after process: node:{:?}, lr: {:?}\n", node, lr);
            }

            if node.right.is_some() {
                return process(Rc::clone(node.right.as_ref().unwrap()))
            }
        }
        return root
    }
    if let Some(node) = root { process(Rc::clone(&node)); };
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use utils::build_tree;

    #[test]
    fn it_works_01() {
        let nil = std::i32::MIN;
        let mut input = build_tree(vec![1,2,3,4,5,nil,6]);
        // println!("input: {:?}.", input);
        flatten(&mut input);
        println!("{:?}", input);
    }
}
