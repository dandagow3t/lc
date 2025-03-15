pub struct Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;

        envelopes.sort_unstable_by_key(|e| (e[0], -e[1]));

        let mut dp: Vec<i32> = Vec::with_capacity(envelopes.len());

        for envelope in envelopes {
            let height = envelope[1];
            if let Err(index) = dp.binary_search(&height) {
                if index == dp.len() {
                    dp.push(height);
                } else {
                    dp[index] = height;
                }
            }
        }

        dp.len() as i32
    }
}