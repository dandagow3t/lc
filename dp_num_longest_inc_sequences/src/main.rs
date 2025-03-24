// https://leetcode.com/problems/number-of-longest-increasing-subsequence/description/
// 673. Number of Longest Increasing Subsequence
// Given an integer array nums, return the number of longest increasing subsequences.
//
// Notice that the sequence has to be strictly increasing.
//
//
//
// Example 1:
//
// Input: nums = [1,3,5,4,7]
// Output: 2
// Explanation: The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].
// Example 2:
//
// Input: nums = [2,2,2,2,2]
// Output: 5
// Explanation: The length of the longest increasing subsequence is 1, and there are 5 increasing subsequences of length 1, so output 5.
//
//
// Constraints:
//
// 1 <= nums.length <= 2000
// -106 <= nums[i] <= 106
// The answer is guaranteed to fit inside a 32-bit integer.
use dp_num_longest_inc_sequences::solution::Solution;


fn main() {
    println!("solution {}", Solution::find_number_of_lis(vec![1,3,5,4,7]));
    println!("solution {}", Solution::find_number_of_lis(vec![2,2,2,2,2]));
    println!("solution {}", Solution::find_number_of_lis(vec![1,2,4,3,5,4,7,2]));
    println!("solution {}", Solution::find_number_of_lis_v2(vec![1,1,1,2,2,2,3,3,3]));


}