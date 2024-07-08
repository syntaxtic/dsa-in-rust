struct Solution;

fn main() {
    let mut input = vec![1, 2, 3, 3, 4, 4, 4, 5];
    let result = Solution::remove_duplicates(&mut input);
    println!("{:?}", result);
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut write = 0;
        let mut read = 1;

        while read < nums.len() {
            if nums[read] == nums[write] {
                read += 1;
            } else {
                write += 1;
                nums[write] = nums[read];
            }
        }

        (write + 1) as i32
    }

    // pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    //     nums.dedup();
    //     nums.len() as i32
    // }
}
