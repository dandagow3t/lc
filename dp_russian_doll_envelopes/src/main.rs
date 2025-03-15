// https://leetcode.com/problems/russian-doll-envelopes/
// 354. Russian Doll Envelopes
// You are given a 2D array of integers envelopes where envelopes[i] = [wi, hi] represents the width and the height of an envelope.
// One envelope can fit into another if and only if both the width and height of one envelope are greater than the other envelope's width and height.
//
// Return the maximum number of envelopes you can Russian doll (i.e., put one inside the other).
//
// Note: You cannot rotate an envelope.
//
// Example 1:
//
// Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
// Output: 3
// Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
// Example 2:
//
// Input: envelopes = [[1,1],[1,1],[1,1]]
// Output: 1
//
// Constraints:
//
// 1 <= envelopes.length <= 105
// envelopes[i].length == 2
// 1 <= wi, hi <= 105

use dp_russian_doll_envelopes::dp::Solution;
fn main() {

    println!("{}", Solution::max_envelopes(vec![[30,50].to_vec(),[12,2].to_vec(),[3,4].to_vec(),[12,15].to_vec()])) // 3
    // println!("{}", Solution::max_envelopes_v2_time_limit_exceeded(vec![[1,1].to_vec(), [1,1].to_vec(), [1,1].to_vec()])) // 1
    // println!("{}", Solution::max_envelopes(vec![[1,3].to_vec(),[3,5].to_vec(),[6,7].to_vec(),[6,8].to_vec(),[8,4].to_vec(),[9,5].to_vec()])); // 3
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_envelopes(vec![[5, 4].to_vec(), [6, 4].to_vec(), [6, 7].to_vec(), [2, 3].to_vec()]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_envelopes(vec![[1,1].to_vec(), [1,1].to_vec(), [1,1].to_vec()]), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_envelopes(vec![[30,50].to_vec(),[12,2].to_vec(),[3,4].to_vec(),[12,15].to_vec()]), 3);
    }
}
