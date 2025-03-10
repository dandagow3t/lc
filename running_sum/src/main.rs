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

fn main() {
    let nums = [1,2,3,4];
    println!("{:?}", Solution::running_sum(nums.to_vec()));
    let nums = [1,1,1,1, 1];
    println!("{:?}", Solution::running_sum(nums.to_vec()));
    let nums = [3,1,2,10,1];
    println!("{:?}", Solution::running_sum(nums.to_vec()));
}
