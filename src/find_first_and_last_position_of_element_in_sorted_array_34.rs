
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
            
    vec![1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(search_range(vec![5,7,7,8,8,10], 8), vec![3,4]);
    }
}
