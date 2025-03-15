pub struct Solution;

impl Solution {


    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        if envelopes.len() == 0 {
            return 0;
        }

        if envelopes.len() == 1 {
            return 1;
        }

        let mut max_sequence: Vec<Vec<i32>> = Vec::new();
        for i in 0..envelopes.len() {
            let mut sequence = Vec::new();
            sequence.push(envelopes[i].clone());
            let mut w = envelopes[i][0];
            let mut l = envelopes[i][1];
            for j in 0..envelopes.len() {
                if i != j && envelopes[j][0] < w && envelopes[j][1] < l {
                    sequence.push(envelopes[j].clone());
                    w = envelopes[j][0];
                    l = envelopes[j][1];
                }
            }
            println!("seq {:?}", sequence);
            if max_sequence.len() < sequence.len() {
                max_sequence = sequence;
            }
            println!("max {:?}", max_sequence);
        }
        max_sequence.len() as i32
    }
}