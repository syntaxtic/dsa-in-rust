struct Solution;

fn main() {
    let input = vec![7, 1, 5, 3, 6, 4];
    let result = Solution::contains_duplicate(input);
    println!("{:?}", result);
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = std::collections::HashSet::new();
        for num in nums.into_iter() {
            if !seen.insert(num) {
                return true;
            }
        }
        false
    }
}
