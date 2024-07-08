struct Solution;

fn main() {
    let mut input = vec![7, 1, 5, 3, 6, 4];
    let result = Solution::rotate(&mut input, 3);
    println!("{:?}", input);
}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let copy = nums.clone();
        let length = nums.len();
        let shift = k as usize % length;

        for i in 0..nums.len() {
            nums[i] = copy[(length - shift + i) % length]
        }
    }

    pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
        let shift = k as usize % nums.len();
        nums.rotate_right(shift);
    }
}
