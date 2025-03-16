pub struct Solution;
impl Solution {
    // This solution has O(n^2) time complexity
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // implement the solution using dynamic programming
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut prev: Vec<i32> = vec![-1; n];
        for i in 0..n {
            for j in 0..i {
                // here is the core logic
                if nums[i] > nums[j] {
                    if dp[i] < dp[j] + 1 {
                        dp[i] = dp[j] + 1;
                        prev[i] = j as i32;
                    }
                    // dp[i] = dp[i].max(dp[j] + 1);
                }
                // println!("{}:{}, dp {:?}", i, j, dp);
            }
        }

        let mut next_index = dp.iter().enumerate().max_by_key(|&(_, v)| v).unwrap().0;
        let solution = *dp.iter().max().unwrap();
        let mut sub_seq = Vec::with_capacity(solution as usize);
        for i in 0..solution {
            sub_seq.push(nums[next_index as usize]);
            next_index = prev[next_index as usize] as usize;
        }
        sub_seq.reverse();
        println!("sub_seq {:?}", sub_seq);

        solution
    }

    // This solution has O(nlogn) time complexity
    pub fn length_of_lis_tail(nums: Vec<i32>) -> i32 {
        let mut tails = Vec::new();
        for num in nums {
            println!("num {:?}", num);
            if let Err(index) = tails.binary_search(&num) {
                if index == tails.len() {
                    tails.push(num);
                } else {
                    tails[index] = num;
                }
                println!("index {}", index);
                println!("tails {:?}", tails);
            }
        }

        println!("tails {:?}", tails);
        tails.len() as i32
    }
}