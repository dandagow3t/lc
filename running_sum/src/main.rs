// https://leetcode.com/problems/running-sum-of-1d-array/
// 1480. Running Sum of 1d Array
// Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).
//
// Return the running sum of nums.
//
// Example 1:
//
// Input: nums = [1,2,3,4]
// Output: [1,3,6,10]
// Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
// Example 2:
//
// Input: nums = [1,1,1,1,1]
// Output: [1,2,3,4,5]
// Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
// Example 3:
//
// Input: nums = [3,1,2,10,1]
// Output: [3,4,6,16,17]
//
// Constraints:
//
// 1 <= nums.length <= 1000
// -10^6 <= nums[i] <= 10^6

struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut running_sum = Vec::with_capacity(nums.len());
        for n in nums {
            running_sum.push(n + running_sum.last().unwrap_or(&0));
        }
        running_sum
    }
}

// Transform main into tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [1,2,3,4];
        assert_eq!(Solution::running_sum(nums.to_vec()), vec![1,3,6,10]);
    }

    #[test]
    fn test_2() {
        let nums = [1,1,1,1, 1];
        assert_eq!(Solution::running_sum(nums.to_vec()), vec![1,2,3,4,5]);
    }

    #[test]
    fn test_3() {
        let nums = [3,1,2,10,1];
        assert_eq!(Solution::running_sum(nums.to_vec()), vec![3,4,6,16,17]);
    }
}
