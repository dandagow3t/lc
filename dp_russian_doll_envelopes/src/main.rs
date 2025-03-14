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
struct Solution;

impl Solution {


    // pub fn max_envelopes_v1_not_ok(envelopes: Vec<Vec<i32>>) -> i32 {
    //     if envelopes.len() == 0 {
    //         return 0;
    //     }
    //
    //     if envelopes.len() == 1 {
    //         return 1;
    //     }
    //
    //     let mut max_sequence: Vec<Vec<i32>> = Vec::new();
    //     for i in 0..envelopes.len() {
    //         let mut sequence = Vec::new();
    //         sequence.push(envelopes[i].clone());
    //         let mut w = envelopes[i][0];
    //         let mut l = envelopes[i][1];
    //         for j in 0..envelopes.len() {
    //             if i != j && envelopes[j][0] < w && envelopes[j][1] < l {
    //                 sequence.push(envelopes[j].clone());
    //                 w = envelopes[j][0];
    //                 l = envelopes[j][1];
    //             }
    //         }
    //         println!("seq {:?}", sequence);
    //         if max_sequence.len() < sequence.len() {
    //             max_sequence = sequence;
    //         }
    //         println!("max {:?}", max_sequence);
    //     }
    //     max_sequence.len() as i32
    // }

    fn build_sequence_v2(envelope: Vec<i32>, envelopes: Vec<Vec<i32>>, prefix: String) -> i32 {
        let label = format!("{}/{:?}", prefix, &envelope);
        // println!("{} envelopes {:?}", &label, &envelopes);
        let mut max = 0;
        for i in 0..envelopes.len() {
            if envelope[0] > envelopes[i][0] && envelope[1] > envelopes[i][1] {
                let next_envelope = envelopes[i].clone();

                let next_envelopes = envelopes.iter().filter(|&x| x[0] != next_envelope[0] || x[1] != next_envelope[1]).cloned().collect();
                let next_sequence_len = Self::build_sequence_v2(next_envelope, next_envelopes, label.clone());
                if max < next_sequence_len {
                    max = next_sequence_len;
                }
            }
        }
        let result = max + 1;
        // println!("{} max {}", label, result);
        result
    }

    pub fn max_envelopes_v2_time_limit_exceeded(envelopes: Vec<Vec<i32>>) -> i32 {
        if envelopes.len() == 0 {
            return 0;
        }

        if envelopes.len() == 1 {
            return 1;
        }

        let mut max = 0;
        for i in 0..envelopes.len() {
            let next_envelope = envelopes[i].clone();
            let next_envelopes: Vec<Vec<i32>> = envelopes.iter().filter(|&x| x[0] != next_envelope[0] || x[1] != next_envelope[1]).cloned().collect();
            if next_envelopes.len() > 0 {
                let seq_len = Self::build_sequence_v2(next_envelope, next_envelopes, String::from("  "));
                if max < seq_len {
                    max = seq_len;
                }
            }

        }
        return if max == 0 { 1 } else { max };
    }

}

fn main() {

    // println!("{}", Solution::max_envelopes(vec![[30,50].to_vec(),[12,2].to_vec(),[3,4].to_vec(),[12,15].to_vec()])) // 3
    println!("{}", Solution::max_envelopes_v2_time_limit_exceeded(vec![[1,1].to_vec(), [1,1].to_vec(), [1,1].to_vec()])) // 1
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
