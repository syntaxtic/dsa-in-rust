struct Solution;

fn main() {
    let input1 = vec![1, 2, 2, 1];
    let input2 = vec![2, 2];
    let result = Solution::intersect(input1, input2);
    println!("{:?}", result);
}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut counts = std::collections::HashMap::new();
        let mut result = vec![];

        for i in 0..nums1.len() {
            *counts.entry(nums1[i]).or_insert(0) += 1;
        }

        for i in 0..nums2.len() {
            if counts.contains_key(&nums2[i]) {
                result.push(nums2[i]);
                *counts.entry(nums2[i]).or_insert(0) -= 1;
                if counts.get(&nums2[i]) == Some(&0) {
                    counts.remove(&nums2[i]);
                }
            }
        }

        result
    }
}
