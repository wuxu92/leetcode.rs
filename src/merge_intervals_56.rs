// Definition for an interval.
#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
  pub start: i32,
  pub end: i32,
}

impl Interval {
  #[inline]
  pub fn new(start: i32, end: i32) -> Self {
    Interval {
      start,
      end
    }
  }
}

impl Clone for Interval {
    #[inline]
    fn clone(&self) -> Interval { Interval::new(self.start, self.end) }
}

pub fn merge(intervals: Vec<Interval>) -> Vec<Interval> {
    // sort and merge
    let mut res = Vec::new();
    let mut copy = intervals.clone();
    copy.sort_unstable_by(|a, b| b.start.cmp(&a.start) );
    if let Some(mut cur) = copy.pop() {
        while let Some(next) = copy.pop() {
            if next.start <= cur.end {
                if next.end > cur.end { cur.end = next.end;}
            } else {
                res.push(Interval::new(cur.start, cur.end));
                cur = next;
            }
        }
        res.push(cur);  // push the last one
    }
    res
}

fn ii(i:i32, j:i32) -> Interval { Interval::new(i, j) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let ins = vec![ii(1,3),ii(2,6),ii(8,10),ii(15,18)];
        assert_eq!(merge(ins), vec![ii(1,6),ii(8,10),ii(15,18)]);
    }
    #[test]
    fn it_works_2() {
        let ins = vec![ii(1,4),ii(4,5)];
        assert_eq!(merge(ins), vec![ii(1,5)]);
    }
    #[test]
    fn it_works_3() {
        let ins = vec![ii(1,5)];
        assert_eq!(merge(ins), vec![ii(1,5)]);
    }
    #[test]
    fn it_works_4() {
        let ins = vec![ii(1,2),ii(4,5)];
        assert_eq!(merge(ins), vec![ii(1,2), ii(4,5)]);
    }
}
