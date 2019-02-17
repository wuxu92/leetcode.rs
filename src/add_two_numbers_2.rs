// @url https://leetcode.com/problems/add-two-numbers/
//
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>
}

impl ListNode {
	#[inline]
	fn new(val: i32) -> Self {
		ListNode {
			next: None,
			val
		}
	}
}


#[allow(dead_code)]
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut obj : Vec<i32> = Vec::new();

    let mut res : Option<Box<ListNode>> = None;
    let mut carry = 0;
    loop {
        if l1.is_none() && l2.is_none() { break; }
        if let Some(n1) = &l1 { carry += n1.val; }
        if let Some(n2) = &l2 { carry += n2.val; }
        obj.push(carry%10);
        carry = carry/10;
        if l1 != None { l1 = l1.unwrap().next; }
        if l2 != None { l2 = l2.unwrap().next; }
    }
    if carry == 1 { obj.push(1); }
    // reverse
    for &i in obj.iter().rev() {
        let mut item = Box::new(ListNode::new(i));
        item.next = res.take();
        res = Some(item);
    }
    res
}


// build by reverse
#[allow(dead_code)]
fn build_list(v : Vec<i32>) -> Option<Box<ListNode>> {
    let mut obj : Option<Box<ListNode>> = Option::None;
    for &i in v.iter().rev() {
        let mut item = Box::new(ListNode::new(i));
        item.next = obj.take();
        obj = Some(item);
    }
    obj
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(build_list(vec![1,2,3]), Some(Box::new(ListNode{val: 1,
            next: Some(Box::new(ListNode{val: 2, 
                next: Some(Box::new(ListNode::new(3)))
            }))
        })));
    }
    #[test]
    fn it_works_02() {
        let l1 = build_list(vec![2,4,3]);
        let l2 = build_list(vec![5,6,4]);
        let res = build_list(vec![7,0,8]);
        assert_eq!(add_two_numbers(l1, l2), res);
    }
    #[test]
    fn it_works_03() {
        let l1 = build_list(vec![2,4,3]);
        let l2 = build_list(vec![5,6,4,5,6,7]);
        let res = build_list(vec![7,0,8,5,6,7]);
        assert_eq!(add_two_numbers(l1, l2), res);
    }
    #[test]
    fn it_works_04() {
        let l1 = build_list(vec![2,4,0,7,8,9]);
        let l2 = build_list(vec![5,6,4]);
        let res = build_list(vec![7,0,5,7,8,9]);
        assert_eq!(add_two_numbers(l1, l2), res);
    }
}
