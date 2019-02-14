
// @url https://leetcode.com/problems/combination-sum/

// Right without sort:
pub fn combination_sum_1(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
	use std::collections::vec_deque::VecDeque;
	let mut vec_deq = VecDeque::new();
	let mut res = Vec::new();
	for &item in candidates.iter() {
		if item == target { res.push(vec![item]); }
		vec_deq.push_back(vec![item]);
	}

	let sum = |v: &Vec<i32>| -> i32 {
		let mut sum = 0;
		for i in v { sum += i;}
		sum
	};

	loop {
		if let Some(vec) = vec_deq.pop_front() {
			let s = sum(&vec);
			// 总是升序的
			let min = vec.last().unwrap();
			for item in candidates.iter() {
				if item < min { continue; }
				let add = s + item;
				if add > target { continue ;} // if larger than target, skip it
				else {
					let mut new_vec=vec.clone();
					new_vec.push(*item);
					if add == target { res.push(new_vec);}
					else { vec_deq.push_back(new_vec); }
				}
			}
		} else {break;}
	}
	res
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::collections::vec_deque::VecDeque;
    let mut vec_deq = VecDeque::new();
    let mut res = Vec::new();
    let mut copy = candidates.clone();
    copy.sort_unstable();
    for &item in copy.iter() {
        if item == target { res.push(vec![item]); break; }
        vec_deq.push_back(vec![item, item]);  // save item and sum in vec
    }

    loop {
        if let Some(vec) = vec_deq.pop_front() {
            let mut s = vec.last().unwrap();
            let min = vec[vec.len()-2]; // always asend
            for &item in copy.iter() {
                if item <  min { continue; }
                let add = s + item;
                if add > target { break; } // if larger than target, skip it
                else {
                    let mut new_vec=vec.clone();
                    new_vec.pop(); // remove last item which is sum 
                    new_vec.push(item);
                    if add == target { res.push(new_vec);}
                    else { new_vec.push(add); vec_deq.push_back(new_vec); }
                }
            }
        } else {break;}
    }
    res
}

#[cfg(test)]
/// the output vec! order may different cause assert failed, which still pass the online judge
mod tests {
    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(combination_sum(vec![2,3,6,7], 7), vec![vec![7], vec![2,2,3]]);
    }
    #[test]
    fn it_works_2() {
        assert_eq!(combination_sum(vec![2,3,5], 8), vec![vec![3,5], vec![2,3,3], vec![2,2,2,2]]);
    }
    #[test]
    fn it_works_3() {
        assert_eq!(combination_sum(vec![11,6,5,8,3,12,7,4], 12), vec![vec![12], vec![6, 6], vec![5, 7], vec![4, 8], vec![3, 3, 6], vec![3, 4, 5], vec![4, 4, 4], vec![3, 3, 3, 3]])
    }
    #[test]
    fn it_works_4() {
        assert_eq!(combination_sum(vec![2], 2), vec![vec![2]]);
    }
    #[test]
    fn it_works_5() {
        let v:Vec<Vec<i32>> = Vec::new();
        assert_eq!(combination_sum(vec![2], 5), v);
    }
    #[test]
    fn it_works_6() {
        let v:Vec<Vec<i32>> = Vec::new();
        assert_eq!(combination_sum(vec![], 5), v);
    }
}
