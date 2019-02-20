// @url https://leetcode.com/problems/sort-list/

use utils::ListNode;

// use pure box not work out
/*
#[allow(dead_code)]
pub fn sort_list_1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    use std::rc::Rc;
    if head == None { return None }

    // use pile
    let res : Rc<Box<ListNode>> = Rc::new(Box::new(ListNode::new(0)));
    let mut h = head;
    // loop head
    loop {
        if h.is_none() { break }
        let cur = h.unwrap();
        let val = cur.val;
        // rc: ListNode
        let mut rc = Rc::get_mut(&mut Rc::clone(&res)).unwrap();
        let mut new_node = Box::new(ListNode::new(val));
        // loop res
        loop {
            if rc.next.is_none() {
                rc.next = Some(new_node);
                break;
            } else {
                let next_val = rc.next.as_ref().unwrap().val;
                if next_val > val {
                    // insert before
                    new_node.next = rc.next.take();
                    rc.next = Some(new_node);
                    break;
                }
                rc = &mut rc.next.unwrap();
            }
        }
        h = cur.next;
        
    }
	None
}
*/

// use vec to sort
#[allow(dead_code)]
pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vec : Vec<i32> = vec![std::i32::MAX];
    let mut h = head;
    while let Some(cur) = h {
        let val = cur.val;
        for idx in 0..vec.len() {
            if val <= vec[idx] { vec.insert(idx, val); break }
        }
        h = cur.next;
    }
    // build list from vec
    vec.pop();
    let mut res : Option<Box<ListNode>> = None;
    for v in vec.iter().rev() {
        res = Some(Box::new(ListNode {val: *v, next: res }));
    }
    res
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use utils::build_listnode;
    #[test]
    fn it_works_01() {
        let input = build_listnode(vec![4,2,1,3]);
        let res = build_listnode(vec![1,2,3,4]);
        assert_eq!(sort_list(input), res);
    }
    #[test]
    fn it_works_02() {
        let input = build_listnode(vec![-1,5,3,4,0]);
        let res = build_listnode(vec![-1,0,3,4,5]);
        assert_eq!(sort_list(input), res);
    }
    #[test]
    fn it_works_03() {
        let input = build_listnode(vec![]);
        let res = build_listnode(vec![]);
        assert_eq!(sort_list(input), res);
    }
}
