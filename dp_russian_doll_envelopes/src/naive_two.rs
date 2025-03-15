pub struct Solution;

impl Solution {

    fn build_sequence(envelope: Vec<i32>, envelopes: Vec<Vec<i32>>, prefix: String) -> i32 {
        let label = format!("{}/{:?}", prefix, &envelope);
        // println!("{} envelopes {:?}", &label, &envelopes);
        let mut max = 0;
        for i in 0..envelopes.len() {
            if envelope[0] > envelopes[i][0] && envelope[1] > envelopes[i][1] {
                let next_envelope = envelopes[i].clone();

                let next_envelopes = envelopes.iter().filter(|&x| x[0] != next_envelope[0] || x[1] != next_envelope[1]).cloned().collect();
                let next_sequence_len = Self::build_sequence(next_envelope, next_envelopes, label.clone());
                if max < next_sequence_len {
                    max = next_sequence_len;
                }
            }
        }
        let result = max + 1;
        // println!("{} max {}", label, result);
        result
    }

    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
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
                let seq_len = Self::build_sequence(next_envelope, next_envelopes, String::from("  "));
                if max < seq_len {
                    max = seq_len;
                }
            }

        }
        return if max == 0 { 1 } else { max };
    }

}