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
use dp_longest_increasing::solution::Solution;


fn main() {
    // println!("solution {}", Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]));
    // println!("solution {}", Solution::length_of_lis_tail(vec![10,9,2,5,3,7,101,18]));
    println!("solution {}", Solution::length_of_lis(vec![5, 9, 2, 1, 4, 6, 3, 2, 9]));
    // println!("solution {}", Solution::length_of_lis_tail(vec![10, 9, 2, 5, 3, 7, 101, 18]));
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