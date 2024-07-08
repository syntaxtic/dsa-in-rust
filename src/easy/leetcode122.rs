struct Solution;

fn main() {
    let input = vec![7, 1, 5, 3, 6, 4];
    let result = Solution::max_profit(input);
    println!("{:?}", result);
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
        }

        profit
    }
}
