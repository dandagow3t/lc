// https://leetcode.com/problems/largest-divisible-subset/
// 368. Largest Divisible Subset
// Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:
//
// answer[i] % answer[j] == 0, or
// answer[j] % answer[i] == 0
// If there are multiple solutions, return any of them.
//
// Example 1:
//
// Input: nums = [1,2,3]
// Output: [1,2]
// Explanation: [1,3] is also accepted.
// Example 2:
//
// Input: nums = [1,2,4,8]
// Output: [1,2,4,8]
//
//
// Constraints:
//
// 1 <= nums.length <= 1000
// 1 <= nums[i] <= 2 * 109
// All the integers in nums are unique.
//
struct Solution;
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums; // Create a mutable copy of the input vector
        nums.sort_unstable(); // Sort the numbers in ascending order for divisibility checks

        let n = nums.len(); // Get the length of the sorted vector
        if n == 0 { // If the vector is empty
            return Vec::new(); // Return an empty vector
        }

        let mut dp = vec![1; n]; // Initialize a DP vector with 1s, where dp[i] stores the size of the largest divisible subset ending at nums[i]
        let mut prev = vec![-1; n]; // Initialize a vector to store the previous index in the largest divisible subset for each element, -1 indicates no previous element
        let mut max_index = 0; // Initialize the index of the element with the largest divisible subset to 0

        for i in 1..n { // Iterate through the sorted vector starting from the second element
            println!("i:{}", i);
            for j in 0..i { // Iterate through the elements before the current element
                println!("j:{}", j);
                if nums[i] % nums[j] == 0 { // Check if nums[i] is divisible by nums[j]
                    if dp[i] < dp[j] + 1 { // If adding nums[i] to the subset ending at nums[j] creates a larger subset
                        dp[i] = dp[j] + 1; // Update dp[i] with the new size of the subset
                        prev[i] = j as i32; // Store the index j as the previous index in the subset ending at nums[i]
                    }
                }
                println!("dp: {:?}", dp);
            }
            if dp[i] > dp[max_index] { // If the size of the subset ending at nums[i] is larger than the current maximum
                max_index = i; // Update max_index to the index of the new largest subset
            }
            println!("max_index:{}", max_index);
            println!("prev: {:?}", prev);
        }

        let mut result = Vec::new(); // Initialize an empty vector to store the result
        let mut current = max_index as i32; // Initialize current index to the index of the largest subset

        while current != -1 { // Backtrack through the prev vector to reconstruct the subset
            result.push(nums[current as usize]); // Add the current element to the result vector
            println!("current {}, result: {:?}", current, result);
            current = prev[current as usize]; // Move to the previous element in the subset
        }

        result.reverse(); // Reverse the result vector to get the correct order
        result // Return the largest divisible subset
    }
}

fn main() {
    println!("Solution {:?}", Solution::largest_divisible_subset(vec![5,9,18,54,108,540,90,180,360,720]));
}

// Implement tests
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_largest_divisible_subset1() {
        assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 4, 8]), vec![1, 2, 4, 8]);
    }

    #[test]
    fn test_largest_divisible_subset2() {
        assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 3]), vec![1, 2]);
    }

    #[test]
    fn test_largest_divisible_subset3() {
        assert_eq!(Solution::largest_divisible_subset(vec![1]), vec![1]);
    }
}