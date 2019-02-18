use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
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
        let tree = build_tree(vec![1,std::i32::MIN,2,3]);
        let res = Some(Rc::new(RefCell::new(
                    TreeNode {
                        val: 1,
                        left: None,
                        right: Some(Rc::new(RefCell::new(
                                    TreeNode {
                                        val: 2,
                                        left: Some(Rc::new(RefCell::new(
                                                    TreeNode::new(3)
                                                    ))),
                                        right: None
                                    }
                                    )))
                    }
                    )));
        assert_eq!(tree, res);
    }
}
