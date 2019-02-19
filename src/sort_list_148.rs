// @url https://leetcode.com/problems/sort-list/


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

#[allow(dead_code)]
pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    use std::rc::Rc;
    if head == None { return None }

    // use pile
    let mut res : Rc<Box<ListNode>> = Rc::new(Box::new(ListNode::new(0)));
    let mut h = head.unwrap().next;

    loop {
        if h == None { break }
        let cur = h.unwrap();
        let val = cur.val;
        let rc = Rc::clone(&res);

        loop {
            let new_node = ListNode::new(val);
            if rc.next == None {
                rc.next = Some(Box::new(new_node));
            }
            let next_val = rc.next.unwrap().val;
            if next_val > val {
                // insert before
            }
        }
        
    }
	None
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(2 + 2, 4);
    }
}
