pub struct Solution;
impl Solution {
    // This solution has O(n^2) time complexity
    pub fn find_number_of_lis(mut nums: Vec<i32>) -> i32 {
        // implement the solution using dynamic programming
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut dp_clones = vec![1; n];

        for i in 0..n {
            for j in 0..i {
                // here is the core logic
                if nums[i] > nums[j] {
                    if dp[i] < dp[j] + 1 {
                        dp[i] = dp[j] + 1;
                        dp_clones[i] = dp_clones[j];
                    } else if dp[i] == dp[j] + 1 {
                        dp_clones[i] += dp_clones[j];
                    }
                }
            }
        }

        let max_val = *dp.iter().max().unwrap(); // Find the max value in dp

        dp.iter()
            .enumerate()
            .filter(|&(_, &val)| val == max_val) // Filter indices where dp[index] == max_val
            .map(|(index, _)| dp_clones[index]) // Map to the value at the same index in dp_clones
            .sum() // Sum the values
    }

    pub fn find_number_of_lis_v2(mut nums: Vec<i32>) -> i32 {
        // implement the solution using dynamic programming
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut dp_clones = vec![1; n];
        let mut max_len = 0;
        let mut count = 0;

        for i in 0..n {
            for j in 0..i {
                // here is the core logic
                if nums[i] > nums[j] {
                    if dp[i] < dp[j] + 1 {
                        dp[i] = dp[j] + 1;
                        dp_clones[i] = dp_clones[j];
                    } else if dp[i] == dp[j] + 1 {
                        dp_clones[i] += dp_clones[j];
                    }
                }
            }

            if dp[i] > max_len {
                max_len = dp[i];
                count = dp_clones[i];
            } else if dp[i] == max_len {
                count += dp_clones[i];
            }
        }

        count
    }
}