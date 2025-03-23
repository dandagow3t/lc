// https://leetcode.com/problems/maximum-length-of-pair-chain/description/
// 646 Maximum length of pair chain
//
// You are given an array of n pairs `pairs`, where `pairs[i] = [left_i, right_i] and left_i < right_i`.
//
// A pair p2 = [c, d] follows a pair p1 = [a, b] if b < c. A chain of pairs can be formed in this fashion.
//
// Return the length longest chain which can be formed.
//
// You do not need to use up all the given intervals. You can select pairs in any order.
//
// Example 1:
//
// Input: pairs = [[1,2],[2,3],[3,4]]
// Output: 2
// Explanation: The longest chain is [1,2] -> [3,4].
// Example 2:
//
// Input: pairs = [[1,2],[7,8],[4,5]]
// Output: 3
// Explanation: The longest chain is [1,2] -> [4,5] -> [7,8].
//
//
// Constraints:
//
// n == pairs.length
// 1 <= n <= 1000
// -1000 <= lefti < righti <= 1000

pub struct Solution;


impl Solution {
    // Has O(n^2) time complexity.
    // Runs in 18ms.
    pub fn solve_v1(chain: Vec<[i32; 2]>) -> i32 {
        let mut chain = chain.clone();
        chain.sort_by_key(|e| e[1]);
        let mut dp: Vec<i32> = vec![1; chain.len()];
        let mut pred = vec![-1; chain.len()];

        for i in 0..chain.len() {
            for j in 0..i {
                if chain[i][0] > chain[j][1] {
                    if dp[i] < dp[j] + 1 {
                        dp[i] = dp[j] + 1;
                        pred[i] = j as i32;
                    }
                }
            }
        }

        *dp.iter().enumerate().max_by_key( |&(_,v)|v).unwrap().1
    }

    // Use O(nlog(n)) time complexity.
    // Runs in 8ms.
    pub fn solve_v2(chain: Vec<Vec<i32>>) -> i32 {
        let mut chain = chain.clone();
        chain.sort_by_key(|e| (e[0], e[1]));

        let mut tails: Vec<Vec<i32>> = Vec::new();
        for pair in chain {
            if let Err(index) = tails.binary_search_by(|p| p[1].cmp(&pair[0]) ) {
                println!("{:?} {:?}", index, pair);
                if index == tails.len() {
                    tails.push(pair);
                }
                else {
                    if pair[1] < tails[index][1] {
                        tails[index] = pair;
                    }
                }
            }
        }

        tails.len() as i32
    }

    // Faster, uses Greedy.
    // Runs in 3ms.
    pub fn solve_v3(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_by_key(|p| p[1]); // Sort by end element

        let mut current_end = i32::MIN;
        let mut chain_length = 0;

        for pair in pairs {
            if pair[0] > current_end {
                chain_length += 1;
                current_end = pair[1];
            }
        }

        chain_length
    }

    // This is the fastest solution!
    // It's identical to v3, but it uses direct array access.
    // Runs in 1ms.
    pub fn solve_v4(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|p| p[1]);

        let mut current_end = i32::MIN;
        let mut chain_length = 0;

        let len = pairs.len();
        for i in 0..len {
            let pair = &pairs[i]; // Direct array access
            if pair[0] > current_end {
                chain_length += 1;
                current_end = pair[1];
            }
        }

        chain_length
    }
}