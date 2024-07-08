struct Solution;

fn main() {
    let input = vec![4, 1, 2, 1, 2];
    let result = Solution::single_number(input);
    println!("{:?}", result);
}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut seen = std::collections::HashSet::new();
        for num in &nums {
            if !seen.insert(num) {
                seen.remove(num);
            }
        }
        *seen.into_iter().next().unwrap()
    }

    pub fn single_number2(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |res, curr| res ^ curr)
    }
}
