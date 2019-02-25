// @url https://leetcode.com/problems/implement-trie-prefix-tree/

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
struct Node {
    val : char,
    next: Vec<Rc<RefCell<Node>>>
}

impl Node {
    #[inline]
    #[allow(dead_code)]
    pub fn new(ch: char) -> Self {
        Node {
            val: ch,
            next: vec![]
        }
    }
}

struct Trie {
    // list: Vec<Rc<Node>>
    list: Node
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    #[allow(dead_code)]
    fn new() -> Self {
        // Trie { list: vec![] }
        Trie { list: Node::new(0 as char) }
    }

    /** Inserts a word into the trie. */
    #[allow(dead_code)]
    fn insert(&mut self, word: String) {
        let mut nw = word;
        nw.push(0 as char);
        let mut bytes = nw.chars();
        // if let Some(ch) = bytes.next() {
        //     // .unwrap().next.borrow_mut();
        //     let mut idx = 0;
        //     while idx < self.list.len() {
        //         if self.list[idx].val == ch { break }
        //         idx += 1;
        //     }
        //     if idx == self.list.len() { self.list.push(Rc::new(Node::new(ch))); }
        //  let mut node = Rc::try_unwrap(self.list[idx].clone()).ok().unwrap();
            let mut node = self.list.clone();
            while let Some(ch) = bytes.next() {
                // if ch in next
                // let (vec, mut idx) = (node.next.clone(), 0);
                let mut idx = 0;
                for (id, vec) in node.next.iter().enumerate() {
                    if vec.borrow().val == ch { idx = id; break }
                }
                if idx == node.next.len() {
                    let n = Node::new(ch);
                    node.next.push(Rc::new(RefCell::new(n)));
                }
                // node = Rc::clone(node.next.borrow()[idx]);
                println!("idx: {}, len: {}", idx, node.next.len());
                let unr = Rc::try_unwrap(node.next[idx].clone());
                println!("result: {:?}", unr);
                let unr = unr.ok().unwrap();
                let unr = unr.into_inner();
                node = unr;
            }
            // append and \0 as end
        // }
    }

    /** Returns if the word is in the trie. */
    #[allow(dead_code)]
    fn search(&self, word: String) -> bool {
        let mut nw = word;
        nw.push(0 as char);
        self.starts_with(nw)
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    #[allow(dead_code)]
    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self.list.clone();
        let mut chars = prefix.chars();
        while let Some(ch) = chars.next() {
            let mut idx = 0;
            let len = node.next.len();
            for (id, item) in node.next.iter().enumerate() {
                if item.borrow().val == ch { idx = id; break; }
            }
            if idx == len { return false }
            node = Rc::try_unwrap(node.next[idx].clone()).ok().unwrap().into_inner();
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let mut obj = Trie::new();
        obj.insert(String::from("wuxu"));
        obj.insert(String::from("rust is good!"));
        obj.insert(String::from("Chars::as_str"));
        assert_eq!(obj.search(String::from("wux")), false);
        assert_eq!(obj.starts_with(String::from("wux")), true);
    }
}
