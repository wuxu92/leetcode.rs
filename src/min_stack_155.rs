// @url https://leetcode.com/problems/min-stack/

#[allow(dead_code)]
#[derive(Debug)]
struct MinStack {
    min: i32,
    vec: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    #[inline]
    fn new() -> Self {
        MinStack{ min: std::i32::MAX, vec: vec![] }
    }
    
    fn push(&mut self, x: i32) {
        if x <= self.min {
            self.vec.push(self.min);
            self.min = x;
        }
        self.vec.push(x);
    }
    
    fn pop(&mut self) {
        if let Some(v) = self.vec.pop() {
            if v == self.min { self.min = self.vec.pop().unwrap(); }
        }
    }
    
    fn top(&self) -> i32 {
        if let Some(v) = self.vec.last() { *v } else { 0 }
    }
    
    fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        let mut  minStack = MinStack::new();
        minStack.push(-2);
        minStack.push(0);
        minStack.push(-3);
        let min = minStack.get_min();
        minStack.pop();
        let t1 = minStack.top();
        let m2 = minStack.get_min();
        println!("min: {}, t1: {}, m1: {}, obk: {:?}", min, t1, m2, minStack);
    }
}
