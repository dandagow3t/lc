// https://leetcode.com/problems/longest-increasing-subsequence/
// 300. Longest Increasing Subsequence
// Medium
// Topics
// Companies
// Given an integer array nums, return the length of the longest strictly increasing subsequence.
//
// Example 1:
//
// Input: nums = [10,9,2,5,3,7,101,18]
// Output: 4
// Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
// Example 2:
//
// Input: nums = [0,1,0,3,2,3]
// Output: 4
// Example 3:
//
// Input: nums = [7,7,7,7,7,7,7]
// Output: 1
//
// Constraints:
//
// 1 <= nums.length <= 2500
// -104 <= nums[i] <= 104

struct Solution;
impl Solution {
    // This solution has O(n^2) time complexity
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // implement the solution using dynamic programming
        let n = nums.len();
        let mut dp = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                // here is the core logic
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap()
    }

    // This solution has O(nlogn) time complexity
    pub fn length_of_lis_tail(nums: Vec<i32>) -> i32 {
        let mut tails = Vec::new();

        for num in nums {
            println!("num {:?}", num);
            if let Err(index) = tails.binary_search(&num) {
                if index == tails.len() {
                    tails.push(num);
                } else {
                    tails[index] = num;
                }
                println!("index {}", index);
                println!("tails {:?}", tails);
            }
        }

        println!("tails {:?}", tails);
        tails.len() as i32
    }
}

fn main() {
    // println!("solution {}", Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]));
    // println!("solution {}", Solution::length_of_lis_tail(vec![10,9,2,5,3,7,101,18]));
    println!("solution {}", Solution::length_of_lis_tail(vec![5, 9, 2, 1, 4, 6, 3, 2, 9]));
    println!("solution {}", Solution::length_of_lis_tail(vec![10, 9, 2, 5, 3, 7, 101, 18]));
}

// add tests (write separate tests for each example given)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis_1() {
        assert_eq!(Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]), 4);
    }

    #[test]
    fn test_length_of_lis_2() {
        assert_eq!(Solution::length_of_lis(vec![0,1,0,3,2,3]), 4);
    }

    #[test]
    fn test_length_of_lis_3() {
        assert_eq!(Solution::length_of_lis(vec![7,7,7,7,7,7,7]), 1);
    }

    #[test]
    fn test_length_of_lis_4() {
        assert_eq!(Solution::length_of_lis_tail(vec![10,9,2,5,3,7,101,18]), 4);
    }

    #[test]
    fn test_length_of_lis_5() {
        assert_eq!(Solution::length_of_lis_tail(vec![0,1,0,3,2,3]), 4);
    }

    #[test]
    fn test_length_of_lis_6() {
        assert_eq!(Solution::length_of_lis_tail(vec![7,7,7,7,7,7,7]), 1);
    }
}